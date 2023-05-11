use std::{
    collections::VecDeque,
    io::{self, Error, ErrorKind, Write},
    str::from_utf8_unchecked,
};

use serde_json::{
    ser::{CompactFormatter, Formatter},
    Value,
};

#[derive(Clone, Debug)]
pub(crate) struct ObjectEntry {
    key: Vec<u8>,
    value: Vec<u8>,
    is_key_done: bool,
}

impl ObjectEntry {
    pub(crate) fn new() -> Self {
        Self {
            key: Vec::new(),
            value: Vec::new(),
            is_key_done: false,
        }
    }

    #[inline]
    pub(crate) fn end_key(&mut self) {
        self.is_key_done = true;
    }

    pub(crate) fn reparse_key<'a>(&'a self) -> Value {
        let key_ser = unsafe { from_utf8_unchecked(self.key.as_slice()) };
        let key: Value = serde_json::from_str(key_ser).unwrap();
        key
    }

    #[inline]
    pub(crate) fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.is_key_done {
            self.value.write_all(bytes)?;
        } else {
            self.key.write_all(bytes)?;
        }
        Ok(())
    }

    pub(crate) fn to_writer<'a>(&'a mut self) -> io::Result<impl Write + 'a> {
        if !self.is_key_done {
            Ok(&mut self.key)
        } else {
            Ok(&mut self.value)
        }
    }

    #[inline]
    pub(crate) fn write_out<W>(&self, first: bool, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_object_key(writer, first)?;
        writer.write_all(self.key.as_slice())?;
        CompactFormatter.end_object_key(writer)?;

        CompactFormatter.begin_object_value(writer)?;
        writer.write_all(self.value.as_slice())?;
        CompactFormatter.end_object_value(writer)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Object {
    entries: Vec<ObjectEntry>,
}

impl Object {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn current_entry(&mut self) -> io::Result<&mut ObjectEntry> {
        self.entries.last_mut().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "Object entry requested when entry is not active.",
            )
        })
    }

    #[inline]
    pub(crate) fn start_key(&mut self) {
        self.entries.push(ObjectEntry::new())
    }

    #[inline]
    pub(crate) fn end_key(&mut self) -> io::Result<()> {
        Ok(self.current_entry()?.end_key())
    }

    pub(crate) fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current_entry()?.write(bytes)?)
    }

    pub(crate) fn to_writer<'a>(&'a mut self) -> io::Result<impl Write + 'a> {
        Ok(self.current_entry()?.to_writer()?)
    }

    #[inline]
    pub(crate) fn write_out<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_object(writer)?;

        let mut entries = self.entries.clone();

        entries.sort_by(|a, b| {
            let a_orig = a.reparse_key();
            let a_utf16 = a_orig.as_str().unwrap().encode_utf16();
            let b_orig = b.reparse_key();
            let b_utf16 = b_orig.as_str().unwrap().encode_utf16();
            a_utf16.cmp(b_utf16)
        });

        let mut first = true;
        for entry in entries {
            entry.write_out(first, writer)?;

            first = false;
        }

        CompactFormatter.end_object(writer)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ObjectStack {
    objects: VecDeque<Object>,
}

impl ObjectStack {
    pub(crate) fn new() -> Self {
        Self {
            objects: VecDeque::new(),
        }
    }

    pub(crate) fn current_object(&mut self) -> io::Result<&mut Object> {
        self.objects.front_mut().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "Object requested when object is not active.",
            )
        })
    }

    pub(crate) fn has_current_object(&mut self) -> bool {
        !self.objects.is_empty()
    }

    #[inline]
    pub(crate) fn start_object(&mut self) {
        self.objects.push_front(Object::new())
    }

    #[inline]
    pub(crate) fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let mut object = self.objects.pop_front().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "Object requested when object is not active.",
            )
        })?;

        if self.has_current_object() {
            let mut writer = self.current_object()?.to_writer()?;
            object.write_out(&mut writer)?;
        } else {
            object.write_out(writer)?;
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn start_key(&mut self) -> io::Result<()> {
        Ok(self.current_object()?.start_key())
    }

    #[inline]
    pub(crate) fn end_key(&mut self) -> io::Result<()> {
        Ok(self.current_object()?.end_key()?)
    }

    pub(crate) fn write<'a, W>(&'a mut self, writer: &'a mut W, bytes: &[u8]) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        if self.has_current_object() {
            self.current_object()?.write(bytes)?
        } else {
            writer.write_all(bytes)?;
        }
        Ok(())
    }

    pub(crate) fn to_writer<'a, W>(&'a mut self, writer: &'a mut W) -> io::Result<impl Write + 'a>
    where
        W: Write + ?Sized,
    {
        let writer: EitherWriter<_, _> = if self.has_current_object() {
            let object_writer = self.current_object()?.to_writer()?;
            EitherWriter::Left(object_writer)
        } else {
            EitherWriter::Right(writer)
        };
        Ok(writer)
    }
}

enum EitherWriter<LeftWriter, RightWriter> {
    Left(LeftWriter),
    Right(RightWriter),
}

impl<LeftWriter, RightWriter> Write for EitherWriter<LeftWriter, RightWriter>
where
    LeftWriter: Write,
    RightWriter: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            EitherWriter::Left(writer) => writer.write(buf),
            EitherWriter::Right(writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            EitherWriter::Left(writer) => writer.flush(),
            EitherWriter::Right(writer) => writer.flush(),
        }
    }
}
