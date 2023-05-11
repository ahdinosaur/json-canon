use std::{
    collections::VecDeque,
    io::{self, Error, ErrorKind, Write},
    str::from_utf8_unchecked,
};

use serde_json::ser::{CompactFormatter, Formatter};

#[derive(Clone, Debug)]
pub(crate) struct ObjectEntry {
    key_orig: Vec<u8>,
    key_ser: Vec<u8>,
    value_ser: Vec<u8>,
    is_key_done: bool,
}

impl ObjectEntry {
    pub(crate) fn new() -> Self {
        Self {
            key_orig: Vec::new(),
            key_ser: Vec::new(),
            value_ser: Vec::new(),
            is_key_done: false,
        }
    }

    #[inline]
    pub(crate) fn cmpable<'a>(&'a self) -> impl Iterator<Item = impl Ord + 'a> {
        let key_orig = unsafe { from_utf8_unchecked(self.key_orig.as_slice()) };
        key_orig.encode_utf16()
    }

    pub(crate) fn write_orig(&mut self, bytes: &[u8]) -> io::Result<()> {
        if !self.is_key_done {
            self.key_orig.write_all(bytes)?;
        }
        Ok(())
    }

    pub(crate) fn write_ser(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.is_key_done {
            self.value_ser.write_all(bytes)?;
        } else {
            self.key_ser.write_all(bytes)?;
        }
        Ok(())
    }

    pub(crate) fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.is_key_done {
            self.value_ser.write_all(bytes)?;
        } else {
            self.key_orig.write_all(bytes)?;
            self.key_ser.write_all(bytes)?;
        }
        Ok(())
    }

    pub(crate) fn to_ser_writer<'a>(&'a mut self) -> io::Result<impl Write + 'a> {
        if !self.is_key_done {
            Ok(&mut self.key_ser)
        } else {
            Ok(&mut self.value_ser)
        }
    }

    #[inline]
    pub(crate) fn end_key(&mut self) {
        self.is_key_done = true;
    }

    #[inline]
    pub(crate) fn write_out<W>(&self, first: bool, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_object_key(writer, first)?;
        writer.write_all(self.key_ser.as_slice())?;
        CompactFormatter.end_object_key(writer)?;

        CompactFormatter.begin_object_value(writer)?;
        writer.write_all(self.value_ser.as_slice())?;
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

    pub(crate) fn write_orig(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current_entry()?.write_orig(bytes)?)
    }

    pub(crate) fn write_ser(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current_entry()?.write_ser(bytes)?)
    }

    pub(crate) fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        Ok(self.current_entry()?.write(bytes)?)
    }

    pub(crate) fn to_ser_writer<'a>(&'a mut self) -> io::Result<impl Write + 'a> {
        Ok(self.current_entry()?.to_ser_writer()?)
    }

    #[inline]
    pub(crate) fn write_out<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_object(writer)?;

        let mut entries = self.entries.clone();

        entries.sort_by(|a, b| a.cmpable().cmp(b.cmpable()));

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
            let mut writer = self.current_object()?.to_ser_writer()?;
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

    pub(crate) fn write_orig(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.has_current_object() {
            self.current_object()?.write_orig(bytes)?;
        };
        Ok(())
    }

    pub(crate) fn write_ser<'a, W>(&'a mut self, writer: &'a mut W, bytes: &[u8]) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        if self.has_current_object() {
            self.current_object()?.write_ser(bytes)?
        } else {
            writer.write_all(bytes)?;
        }
        Ok(())
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

    pub(crate) fn to_ser_writer<'a, W>(
        &'a mut self,
        writer: &'a mut W,
    ) -> io::Result<impl Write + 'a>
    where
        W: Write + ?Sized,
    {
        let writer: ObjectStackWriter<_, _> = if self.has_current_object() {
            let object_writer = self.current_object()?.to_ser_writer()?;
            ObjectStackWriter::Object(object_writer)
        } else {
            ObjectStackWriter::Base(writer)
        };
        Ok(writer)
    }
}

enum ObjectStackWriter<ObjectWriter, BaseWriter> {
    Object(ObjectWriter),
    Base(BaseWriter),
}

impl<ObjectWriter, BaseWriter> Write for ObjectStackWriter<ObjectWriter, BaseWriter>
where
    ObjectWriter: Write,
    BaseWriter: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            ObjectStackWriter::Object(writer) => writer.write(buf),
            ObjectStackWriter::Base(writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            ObjectStackWriter::Object(writer) => writer.flush(),
            ObjectStackWriter::Base(writer) => writer.flush(),
        }
    }
}
