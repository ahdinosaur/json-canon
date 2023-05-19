use std::{
    io::{self, sink, Error, ErrorKind, Write},
    str::from_utf8_unchecked,
};

use serde_json::ser::{CompactFormatter, Formatter};

#[derive(Clone, Debug)]
pub(crate) struct ObjectEntry {
    key: Vec<u8>,
    key_bytes: Vec<u8>,
    value: Vec<u8>,
    is_key_done: bool,
}

impl ObjectEntry {
    pub(crate) fn new() -> Self {
        Self {
            key: Vec::new(),
            key_bytes: Vec::new(),
            value: Vec::new(),
            is_key_done: false,
        }
    }

    #[inline]
    pub(crate) fn end_key(&mut self) {
        self.is_key_done = true;
    }

    pub(crate) fn is_in_key(&mut self) -> bool {
        !self.is_key_done
    }

    #[inline]
    pub(crate) fn cmpable(&self) -> impl Iterator<Item = impl Ord + '_> {
        let key_orig = unsafe { from_utf8_unchecked(self.key_bytes.as_slice()) };
        key_orig.encode_utf16()
    }

    #[inline]
    pub(crate) fn scope(&mut self) -> io::Result<impl Write + '_> {
        if self.is_in_key() {
            Ok(&mut self.key)
        } else {
            Ok(&mut self.value)
        }
    }

    #[inline]
    pub(crate) fn scope_with_key(&mut self) -> io::Result<impl Write + '_> {
        let writer = if self.is_in_key() {
            EitherWriter::Left(BothWriter::new(&mut self.key, &mut self.key_bytes))
        } else {
            EitherWriter::Right(&mut self.value)
        };
        Ok(writer)
    }

    #[inline]
    pub(crate) fn key_bytes(&mut self) -> io::Result<impl Write + '_> {
        let writer = if self.is_in_key() {
            EitherWriter::Left(&mut self.key_bytes)
        } else {
            EitherWriter::Right(sink())
        };
        Ok(writer)
    }

    #[inline]
    pub(crate) fn write_to<W>(&self, first: bool, writer: &mut W) -> io::Result<()>
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
        self.current_entry()?.end_key();
        Ok(())
    }

    #[inline]
    pub(crate) fn is_in_key(&mut self) -> io::Result<bool> {
        let is_in_key = self.current_entry()?.is_in_key();
        Ok(is_in_key)
    }

    #[inline]
    pub(crate) fn scope(&mut self) -> io::Result<impl Write + '_> {
        let writer = self.current_entry()?.scope()?;
        Ok(writer)
    }

    #[inline]
    pub(crate) fn scope_with_key(&mut self) -> io::Result<impl Write + '_> {
        let writer = self.current_entry()?.scope_with_key()?;
        Ok(writer)
    }

    #[inline]
    pub(crate) fn key_bytes(&mut self) -> io::Result<impl Write + '_> {
        let writer = self.current_entry()?.key_bytes()?;
        Ok(writer)
    }

    #[inline]
    pub(crate) fn write_to<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        CompactFormatter.begin_object(writer)?;

        let mut entries = self.entries.clone();

        entries.sort_by(|a, b| a.cmpable().cmp(b.cmpable()));

        let mut first = true;
        for entry in entries {
            entry.write_to(first, writer)?;

            first = false;
        }

        CompactFormatter.end_object(writer)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ObjectStack {
    objects: Vec<Object>,
}

impl ObjectStack {
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub(crate) fn current_object(&mut self) -> io::Result<&mut Object> {
        self.objects.last_mut().ok_or_else(|| {
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
        self.objects.push(Object::new())
    }

    #[inline]
    pub(crate) fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write + ?Sized,
    {
        let mut object = self.objects.pop().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "Object requested when object is not active.",
            )
        })?;

        if self.has_current_object() {
            let mut writer = self.current_object()?.scope()?;
            object.write_to(&mut writer)?;
        } else {
            object.write_to(writer)?;
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn start_key(&mut self) -> io::Result<()> {
        self.current_object()?.start_key();
        Ok(())
    }

    #[inline]
    pub(crate) fn end_key(&mut self) -> io::Result<()> {
        self.current_object()?.end_key()?;
        Ok(())
    }

    #[inline]
    pub(crate) fn is_in_key(&mut self) -> io::Result<bool> {
        let is_in_key = if self.has_current_object() {
            self.current_object()?.is_in_key()?
        } else {
            false
        };
        Ok(is_in_key)
    }

    pub(crate) fn scope<'a, W>(&'a mut self, writer: &'a mut W) -> io::Result<impl Write + 'a>
    where
        W: Write + ?Sized,
    {
        let writer: EitherWriter<_, _> = if self.has_current_object() {
            let object_writer = self.current_object()?.scope()?;
            EitherWriter::Left(object_writer)
        } else {
            EitherWriter::Right(writer)
        };
        Ok(writer)
    }

    pub(crate) fn scope_with_key<'a, W>(
        &'a mut self,
        writer: &'a mut W,
    ) -> io::Result<impl Write + 'a>
    where
        W: Write + ?Sized,
    {
        let writer: EitherWriter<_, _> = if self.has_current_object() {
            let object_writer = self.current_object()?.scope_with_key()?;
            EitherWriter::Left(object_writer)
        } else {
            EitherWriter::Right(writer)
        };
        Ok(writer)
    }

    pub(crate) fn key_bytes(&mut self) -> io::Result<impl Write + '_> {
        let writer: EitherWriter<_, _> = if self.has_current_object() {
            let key_bytes_writer = self.current_object()?.key_bytes()?;
            EitherWriter::Left(key_bytes_writer)
        } else {
            EitherWriter::Right(sink())
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

struct BothWriter<LeftWriter, RightWriter> {
    left: LeftWriter,
    right: RightWriter,
}

impl<LeftWriter, RightWriter> BothWriter<LeftWriter, RightWriter> {
    fn new(left: LeftWriter, right: RightWriter) -> Self {
        Self { left, right }
    }
}

impl<LeftWriter, RightWriter> Write for BothWriter<LeftWriter, RightWriter>
where
    LeftWriter: Write,
    RightWriter: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.left.write_all(buf)?;
        self.right.write_all(buf)?;
        self.flush()?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.left.flush()?;
        self.right.flush()?;
        Ok(())
    }
}
