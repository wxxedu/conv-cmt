use std::{fmt::Display, io::Write};

use super::writer::Writer;

pub trait MyStrTrait<const THRESHOLD: usize>:
    Write + AsRef<str> + AsMut<str> + Sized + Writer + Display
{
    fn new(s: impl AsRef<str>) -> Self;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn clear(&mut self);
}

/// Represents a string that can be either fixed or variable length.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyStr<const THRESHOLD: usize> {
    Fixed(MyStrFixed<THRESHOLD>),
    Variable(String),
}

impl<const THRESHOLD: usize> Default for MyStr<THRESHOLD> {
    fn default() -> Self {
        Self::Fixed(MyStrFixed::default())
    }
}

impl<const THRESHOLD: usize> AsRef<str> for MyStr<THRESHOLD> {
    fn as_ref(&self) -> &str {
        match self {
            Self::Fixed(s) => s.as_ref(),
            Self::Variable(s) => s.as_ref(),
        }
    }
}

impl<const THRESHOLD: usize> AsMut<str> for MyStr<THRESHOLD> {
    fn as_mut(&mut self) -> &mut str {
        match self {
            Self::Fixed(s) => s.as_mut(),
            Self::Variable(s) => s.as_mut(),
        }
    }
}

impl<const THRESHOLD: usize> Write for MyStr<THRESHOLD> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        dbg!(len);
        match self {
            Self::Fixed(s) => {
                if s.offset + len <= THRESHOLD {
                    s.data[s.offset..].copy_from_slice(buf);
                    s.offset += len;
                    Ok(len)
                } else {
                    let mut s = Self::Variable(s.as_ref().to_owned());
                    s.write(buf)?;
                    *self = s;
                    Ok(len)
                }
            }
            Self::Variable(s) => std::str::from_utf8(buf)
                .map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("invalid utf8: {}", e),
                    )
                })
                .map(|x| {
                    s.push_str(x);
                    len
                }),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<const THRESHOLD: usize> Display for MyStr<THRESHOLD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<const THRESHOLD: usize> MyStrTrait<THRESHOLD> for MyStr<THRESHOLD> {
    fn new(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        if s.len() <= THRESHOLD {
            Self::Fixed(MyStrFixed {
                offset: s.len(),
                data: s.as_bytes().try_into().unwrap(),
            })
        } else {
            Self::Variable(s.to_owned())
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Fixed(s) => s.len(),
            Self::Variable(s) => s.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Fixed(s) => s.is_empty(),
            Self::Variable(s) => s.is_empty(),
        }
    }

    fn clear(&mut self) {
        match self {
            Self::Fixed(s) => s.clear(),
            Self::Variable(s) => s.clear(),
        }
    }
}

impl<const THRESHOLD: usize> From<MyStr<THRESHOLD>> for String {
    fn from(s: MyStr<THRESHOLD>) -> Self {
        s.as_ref().to_owned()
    }
}

/// This is a fixed length string that can be allocated on the stack.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MyStrFixed<const MAX_SIZE: usize> {
    offset: usize,
    data: [u8; MAX_SIZE],
}

impl<const THRESHOLD: usize> Default for MyStrFixed<THRESHOLD> {
    fn default() -> Self {
        Self {
            offset: 0,
            data: [0; THRESHOLD],
        }
    }
}

impl<const THRESHOLD: usize> AsRef<str> for MyStrFixed<THRESHOLD> {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&self.data[..self.offset]).unwrap()
    }
}

impl<const THRESHOLD: usize> AsMut<str> for MyStrFixed<THRESHOLD> {
    fn as_mut(&mut self) -> &mut str {
        std::str::from_utf8_mut(&mut self.data[..self.offset]).unwrap()
    }
}

impl<const THRESHOLD: usize> Write for MyStrFixed<THRESHOLD> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let remaining = self.data.len() - self.offset;
        if buf.len() > remaining {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "want to write {} bytes, but only {}/{} bytes available",
                    buf.len(),
                    remaining,
                    self.data.len()
                ),
            ));
        }
        self.data[self.offset..self.offset + buf.len()].copy_from_slice(buf);
        self.offset += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<const THRESHOLD: usize> Display for MyStrFixed<THRESHOLD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<const THRESHOLD: usize> MyStrTrait<THRESHOLD> for MyStrFixed<THRESHOLD> {
    fn new(s: impl AsRef<str>) -> Self {
        let mut this = Self::default();
        this.write_all(s.as_ref().as_bytes()).unwrap();
        this
    }

    fn len(&self) -> usize {
        self.offset
    }

    fn is_empty(&self) -> bool {
        self.offset == 0
    }

    fn clear(&mut self) {
        self.offset = 0;
        self.data.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_str_default() {
        let mut s: MyStr<10> = Default::default();
        assert_eq!(s, MyStr::Fixed(MyStrFixed::default()));
        let bytes_written = s.write(b"abcdefghij");
        assert!(bytes_written.is_ok());
        assert_eq!(bytes_written.unwrap(), 10);
        assert_eq!(s.len(), 10);
        let bytes_written = s.write(b"k");
        assert!(bytes_written.is_ok());
        assert_eq!(bytes_written.unwrap(), 1);
        assert_eq!(s, MyStr::Variable("abcdefghijk".to_owned()));
    }
}
