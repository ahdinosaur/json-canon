use core::mem::replace;
use core::num::FpCategory;
use serde::Serialize;
use serde_json::{
    from_str,
    ser::{CharEscape, CompactFormatter, Formatter},
    Result, Serializer, Value,
};

use std::{io::{self, Error, ErrorKind, Write}, collections::VecDeque};

/// Serialize the given value as a String of JSON.
///
/// Serialization is performed as specified in [RFC 8785](https://tools.ietf.org/html/rfc8785).
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` fails.
#[inline]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let data: Vec<u8> = to_vec(value)?;

    let data: String = unsafe { String::from_utf8_unchecked(data) };

    Ok(data)
}

/// Serialize the given value as a JSON byte vector.
///
/// Serialization is performed as specified in [RFC 8785](https://tools.ietf.org/html/rfc8785).
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` fails.
#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    let mut data: Vec<u8> = Vec::with_capacity(128);

    to_writer(&mut data, value)?;

    Ok(data)
}

/// Serialize the given value as JSON into the IO stream.
///
/// Serialization is performed as specified in [RFC 8785](https://tools.ietf.org/html/rfc8785).
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` fails.
#[inline]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize + ?Sized,
{
    value.serialize(&mut Serializer::with_formatter(
        writer,
        CanonicalFormatter::new(),
    ))
}

#[derive(Clone, Debug)]
struct ObjectEntry {
    key_orig: Vec<u8>,
    key_ser: Vec<u8>,
    value_ser: Vec<u8>,
    is_key_done: bool
}

impl ObjectEntry {
    fn new() -> Self {
        Self {
            key_orig: Vec::new(),
            key_ser: Vec::new(),
            value_ser: Vec::new(),
            is_key_done: false,
        }
    }

    fn push_orig(&mut self, bytes: &[u8]) {
        if !self.is_key_done {
            self.key_orig.extend_from_slice(bytes);
        }
    }

    fn push_ser(&mut self, bytes: &[u8]) {
        if self.is_key_done {
            self.value_ser.extend_from_slice(bytes);
        } else {
            self.key_ser.extend_from_slice(bytes);
        }
    }

    fn get_writer(&mut self) -> Box<dyn Write> {
        if self.is_key_done {
            Box::new(self.value_ser)
        } else {
            Box::new(self.key_ser)
        }
    }

    fn end_key(&mut self) {
        self.is_key_done = true;
    }

    fn write_out<W>(&self, first: bool, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        if !first {
            writer.write_all(",".as_bytes())?;
        }

        writer.write_all(self.key_ser.as_slice())?;
        writer.write_all(":".as_bytes())?;
        writer.write_all(self.value_ser.as_slice())?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ObjectEntries {
    entries: Vec<ObjectEntry>,
}

impl ObjectEntries {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn current(&mut self) -> io::Result<&mut ObjectEntry> {
        self.entries.last_mut().ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Object entry requested when entry is not active.")
        })
    }

    fn start_key(&mut self) {
        self.entries.push(ObjectEntry::new())
    }

    fn end_key(&mut self) -> io::Result<()> {
        Ok(self.current()?.end_key())
    }

    fn push_orig(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current()?.push_orig(bytes))
    }

    fn push_ser(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current()?.push_ser(bytes))
    }

    fn write_out<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_object(writer)?;

        // ...

        CompactFormatter.end_object(writer)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ObjectStack {
    stack: VecDeque<ObjectEntries>,
}

impl ObjectStack {
    fn new() -> Self {
        Self { stack: VecDeque::new() }
    }

    fn current(&mut self) -> io::Result<&mut ObjectEntries> {
        self.stack
            .front_mut()
            .ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "Object requested when object is not active.")
            })
    }
    
    fn has_current(&mut self) -> bool {
        !self.stack.is_empty()
    }

    fn start_object(&mut self) {
        self.stack.push_front(ObjectEntries::new())
    }

    fn start_key(&mut self) -> io::Result<()> {
        Ok(self.current()?.start_key())
    }

    fn end_key(&mut self) -> io::Result<()> {
        Ok(self.current()?.start_key())
    }

    fn push_orig(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current()?.push_orig(bytes)?)
    }

    fn push_ser(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current()?.push_ser(bytes)?)
    }


    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let mut object = self.stack.pop_front().ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Object requested when object is not active.")
        })?;

        let mut writer = if self.has_current() {
            self.current()?.current()?.get_writer()
        } else {
            Box::new(writer)
        };

        object.write_out(&mut writer)
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct CanonicalFormatter {
    stack: ObjectStack,
}

impl CanonicalFormatter {
    pub const fn new() -> Self {
        Self { stack: ObjectStack::new() }
    }
}

impl Formatter for CanonicalFormatter {
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.scope(writer).write_all(b"null")
    }

    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        if value {
            self.scope(writer).write_all(b"true")
        } else {
            self.scope(writer).write_all(b"false")
        }
    }

    fn write_char_escape<W>(&mut self, writer: &mut W, escape: CharEscape) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        static HEX: [u8; 16] = *b"0123456789abcdef";

        let serialized_bytes = match escape {
            CharEscape::Quote => b"\\\"".to_vec(),
            CharEscape::ReverseSolidus => b"\\\\".to_vec(),
            CharEscape::Solidus => b"\\/".to_vec(),
            CharEscape::Backspace => b"\\b".to_vec(),
            CharEscape::FormFeed => b"\\f".to_vec(),
            CharEscape::LineFeed => b"\\n".to_vec(),
            CharEscape::CarriageReturn => b"\\r".to_vec(),
            CharEscape::Tab => b"\\t".to_vec(),
            CharEscape::AsciiControl(control) => {
                [
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX[(control >> 4) as usize],
                    HEX[(control & 0xF) as usize],
                ].to_vec()
            }
        }

        if 
    }

    #[inline]
    fn write_number_str<W>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        todo!("Handle number str (u128/i128)")
    }

    #[inline]
    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        // TOOD: Check
        self.scope(writer).write_all(fragment.as_bytes())
    }

    #[inline]
    fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        // TOOD: Check
        from_str::<Value>(fragment)?
            .serialize(&mut Serializer::with_formatter(
                self.scope(writer),
                Self::new(),
            ))
            .map_err(Into::into)
    }

    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i8(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i16(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i32(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i64(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u8(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u16(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u32(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u64(&mut self.scope(writer), value)
    }

    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.write_float(writer, value.classify(), value)
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.write_float(writer, value.classify(), value)
    }

    #[inline]
    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_string(&mut self.scope(writer))
    }

    #[inline]
    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.end_string(&mut self.scope(writer))
    }

    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_array(&mut self.scope(writer))
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.end_array(&mut self.scope(writer))
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_array_value(&mut self.scope(writer), first)
    }

    #[inline]
    fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.end_array_value(&mut self.scope(writer))
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_object(&mut self.scope(writer))?;

        self.0.push(Entry::new());

        Ok(())
    }

    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let entry: Entry = self
            .0
            .pop()
            .ok_or_else(|| Error::new(ErrorKind::Other, "oh no"))?;

        let mut scope = self.scope(writer);
        let mut first = true;

        let mut keys: Vec<String> = entry
            .object
            .keys()
            .into_iter()
            .map(|k| unsafe { String::from_utf8_unchecked(k.clone()) })
            .collect();

        println!(
            "pre-sort keys: {:?}",
            keys.clone().iter().cloned().collect::<Vec<String>>()
        );

        // sort keys by UTF-16 code units
        // keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));

        keys.sort_by(|a, b| {
            let a_code_units: Vec<u16> = a.encode_utf16().collect();
            println!("a: {}", a);
            println!("a code units: {:?}", a_code_units);
            let b_code_units: Vec<u16> = b.encode_utf16().collect();
            println!("b: {}", b);
            println!("b code units: {:?}", b_code_units);
            a_code_units.cmp(&b_code_units)
        });

        /*
        // sort keys by UTF-16 characters
        keys.sort_by(|a, b| {
            let a_iter = a
                .iter()
                .to_utf8chars()
                .map(|res| res.map(|ch| Into::<Utf16Char>::into(ch)))
                // TODO handle
                .map(|res| res.unwrap());
            let b_iter = b
                .iter()
                .to_utf8chars()
                .map(|res| res.map(|ch| Into::<Utf16Char>::into(ch)))
                // TODO handle
                .map(|res| res.unwrap());
            a_iter.cmp(b_iter)
        });
        */

        println!(
            "post-sort keys: {:?}",
            keys.clone().iter().cloned().collect::<Vec<String>>()
        );

        for key in keys {
            let val = entry
                .object
                .get(key.as_bytes())
                .ok_or_else(|| Error::new(ErrorKind::Other, "oh no"))?;

            CompactFormatter.begin_object_key(&mut scope, first)?;
            scope.write_all(&key.as_bytes())?;
            CompactFormatter.end_object_key(&mut scope)?;

            CompactFormatter.begin_object_value(&mut scope)?;
            scope.write_all(&val)?;
            CompactFormatter.end_object_value(&mut scope)?;

            first = false;
        }

        CompactFormatter.end_object(&mut scope)
    }

    #[inline]
    fn begin_object_key<W>(&mut self, _writer: &mut W, _first: bool) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.entry_mut().map(|entry| {
            entry.complete = false;
        })
    }

    #[inline]
    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.entry_mut().map(|entry| {
            entry.complete = true;
        })
    }

    #[inline]
    fn begin_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        Ok(())
    }

    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let entry: &mut Entry = self.entry_mut()?;

        let key: Vec<u8> = replace(&mut entry.next_key, Vec::new());
        let val: Vec<u8> = replace(&mut entry.next_val, Vec::new());

        entry.object.insert(key, val);

        Ok(())
    }
}

fn write_float<W, F>(
    writer: &mut W,
    category: FpCategory,
    value: F,
) -> io::Result<()>
where
    W: Write + ?Sized,
    F: ryu_js::Float,
{
    match category {
        FpCategory::Nan => Err(Error::new(ErrorKind::InvalidInput, "NaN is not allowed.")),
        FpCategory::Infinite => Err(Error::new(
            ErrorKind::InvalidInput,
            "Infinity is not allowed.",
        )),
        FpCategory::Zero => writer.write_all(b"0"),
        FpCategory::Normal | FpCategory::Subnormal => writer.write_all(ryu_js::Buffer::new().format_finite(value).as_bytes()),
    }
}
