use core::num::FpCategory;
use serde::Serialize;
use serde_json::{
    ser::{CharEscape, CompactFormatter, Formatter},
    Result, Serializer,
};

use std::io::{self, Error, ErrorKind, Write};

use crate::object::ObjectStack;

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
#[repr(transparent)]
pub struct CanonicalFormatter {
    stack: ObjectStack,
}

impl CanonicalFormatter {
    pub fn new() -> Self {
        Self {
            stack: ObjectStack::new(),
        }
    }
}

impl Formatter for CanonicalFormatter {
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let mut writer = self.stack.scope(writer)?;
        writer.write_all(b"null")?;
        Ok(())
    }

    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let mut writer = self.stack.scope(writer)?;
        if value {
            writer.write_all(b"true")?;
        } else {
            writer.write_all(b"false")?;
        }
        Ok(())
    }

    #[inline]
    fn write_char_escape<W>(&mut self, writer: &mut W, escape: CharEscape) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        static HEX_CHARS: [u8; 16] = *b"0123456789abcdef";

        match escape {
            CharEscape::Backspace => {
                self.stack.write_key_orig(&[0x08])?;
                self.stack.scope(writer)?.write_all(b"\\b")?;
            }
            CharEscape::Tab => {
                self.stack.write_key_orig(&[0x09])?;
                self.stack.scope(writer)?.write_all(b"\\t")?;
            }
            CharEscape::LineFeed => {
                self.stack.write_key_orig(&[0x0A])?;
                self.stack.scope(writer)?.write_all(b"\\n")?;
            }
            CharEscape::FormFeed => {
                self.stack.write_key_orig(&[0x0C])?;
                self.stack.scope(writer)?.write_all(b"\\f")?;
            }
            CharEscape::CarriageReturn => {
                self.stack.write_key_orig(&[0x0D])?;
                self.stack.scope(writer)?.write_all(b"\\r")?;
            }
            CharEscape::Quote => {
                self.stack.write_key_orig(&[0x22])?;
                self.stack.scope(writer)?.write_all(b"\\\"")?;
            }
            CharEscape::Solidus => {
                self.stack.write_key_orig(&[0x2F])?;
                self.stack.scope(writer)?.write_all(b"\\/")?;
            }
            CharEscape::ReverseSolidus => {
                self.stack.write_key_orig(&[0x5C])?;
                self.stack.scope(writer)?.write_all(b"\\\\")?;
            }
            CharEscape::AsciiControl(control) => {
                self.stack.write_key_orig(&[control])?;
                self.stack.scope(writer)?.write_all(&[
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX_CHARS[(control >> 4) as usize],
                    HEX_CHARS[(control & 0xF) as usize],
                ])?;
            }
        }
        Ok(())
    }

    #[inline]
    fn write_number_str<W>(&mut self, _writer: &mut W, _value: &str) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        todo!("Handle number str (u128/i128)")
    }

    // https://262.ecma-international.org/10.0/#sec-quotejsonstring
    #[inline]
    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let bytes = fragment.as_bytes();
        self.stack.write_key_orig(bytes)?;
        let mut writer = self.stack.scope(writer)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    #[inline]
    fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let bytes = fragment.as_bytes();
        let mut writer = self.stack.scope(writer)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i8(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i16(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i32(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_i64(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u8(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u16(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u32(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.write_u64(&mut self.stack.scope(writer)?, value)
    }

    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        write_float(&mut self.stack.scope(writer)?, value.classify(), value)
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        write_float(&mut self.stack.scope(writer)?, value.classify(), value)
    }

    #[inline]
    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_string(&mut self.stack.scope(writer)?)
    }

    #[inline]
    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.end_string(&mut self.stack.scope(writer)?)
    }

    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_array(&mut self.stack.scope(writer)?)
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.end_array(&mut self.stack.scope(writer)?)
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_array_value(&mut self.stack.scope(writer)?, first)
    }

    #[inline]
    fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.end_array_value(&mut self.stack.scope(writer)?)
    }

    #[inline]
    fn begin_object<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.stack.start_object();
        Ok(())
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.stack.end_object(writer)
    }

    #[inline]
    fn begin_object_key<W>(&mut self, _writer: &mut W, _first: bool) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.stack.start_key()
    }

    #[inline]
    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        self.stack.end_key()
    }

    #[inline]
    fn begin_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        Ok(())
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        Ok(())
    }
}

fn write_float<W, F>(writer: &mut W, category: FpCategory, value: F) -> io::Result<()>
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
        FpCategory::Normal | FpCategory::Subnormal => {
            writer.write_all(ryu_js::Buffer::new().format_finite(value).as_bytes())
        }
    }
}
