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

    pub(crate) fn cmpable<'a>(&'a self) -> impl Iterator<Item = impl Ord + 'a> {
        let key_orig = unsafe { from_utf8_unchecked(self.key_orig.as_slice()) };
        key_orig.encode_utf16()
    }

    pub(crate) fn get_writer_orig<'a>(&'a mut self) -> Box<dyn Write + 'a> {
        if self.is_key_done {
            Box::new(io::sink())
        } else {
            Box::new(&mut self.key_orig)
        }
    }

    pub(crate) fn get_writer_ser<'a>(&'a mut self) -> Box<dyn Write + 'a> {
        if self.is_key_done {
            Box::new(&mut self.value_ser)
        } else {
            Box::new(&mut self.key_ser)
        }
    }

    pub(crate) fn get_writer<'a>(&'a mut self) -> Box<dyn Write + 'a> {
        if self.is_key_done {
            Box::new(&mut self.value_ser)
        } else {
            Box::new(CombinedWriter::new(&mut self.key_orig, &mut self.key_ser))
        }
    }

    pub(crate) fn end_key(&mut self) {
        self.is_key_done = true;
    }

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

struct CombinedWriter<'a> {
    a: Box<dyn Write + 'a>,
    b: Box<dyn Write + 'a>,
}

impl<'a> CombinedWriter<'a> {
    fn new<A, B>(a: &'a mut A, b: &'a mut B) -> Self
    where
        A: Write + ?Sized,
        B: Write + ?Sized,
    {
        Self {
            a: Box::new(a),
            b: Box::new(b),
        }
    }
}

impl<'a> Write for CombinedWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.a.write(buf)?;
        self.b.write(buf)?;
        // uhhh
        self.flush()?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.a.flush()?;
        self.b.flush()?;
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

    pub(crate) fn start_key(&mut self) {
        self.entries.push(ObjectEntry::new())
    }

    pub(crate) fn end_key(&mut self) -> io::Result<()> {
        Ok(self.current_entry()?.end_key())
    }

    pub(crate) fn get_writer_orig<'a>(&'a mut self) -> io::Result<Box<dyn Write + 'a>> {
        Ok(self.current_entry()?.get_writer_orig())
    }

    pub(crate) fn get_writer_ser<'a>(&'a mut self) -> io::Result<Box<dyn Write + 'a>> {
        Ok(self.current_entry()?.get_writer_ser())
    }

    pub(crate) fn get_writer<'a>(&'a mut self) -> io::Result<Box<dyn Write + 'a>> {
        Ok(self.current_entry()?.get_writer())
    }

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

    pub(crate) fn start_object(&mut self) {
        self.objects.push_front(Object::new())
    }

    pub(crate) fn start_key(&mut self) -> io::Result<()> {
        Ok(self.current_object()?.start_key())
    }

    pub(crate) fn end_key(&mut self) -> io::Result<()> {
        Ok(self.current_object()?.end_key()?)
    }

    pub(crate) fn get_writer_orig<'a>(&'a mut self) -> io::Result<Box<dyn Write + 'a>> {
        let writer = if self.has_current_object() {
            self.current_object()?.get_writer_orig()?
        } else {
            Box::new(io::sink())
        };
        Ok(writer)
    }

    pub(crate) fn get_writer_ser<'a, W>(
        &'a mut self,
        writer: &'a mut W,
    ) -> io::Result<Box<dyn Write + 'a>>
    where
        W: Write + ?Sized,
    {
        let writer = if self.has_current_object() {
            self.current_object()?.get_writer_ser()?
        } else {
            Box::new(writer)
        };
        Ok(writer)
    }

    pub(crate) fn get_writer<'a, W>(
        &'a mut self,
        writer: &'a mut W,
    ) -> io::Result<Box<dyn Write + 'a>>
    where
        W: Write + ?Sized,
    {
        let writer = if self.has_current_object() {
            self.current_object()?.get_writer()?
        } else {
            Box::new(writer)
        };
        Ok(writer)
    }

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

        let mut writer = self.get_writer(writer)?;
        object.write_out(&mut writer)
    }
}
