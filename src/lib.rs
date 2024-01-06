//! # Ansi Stream
//!

use std::{
    io::{self, Cursor, Write},
    ops::{Deref, DerefMut},
};

/// ASCII Escape.
const ESC: u8 = 0x1b;
// foreground colors
pub const FCBLACK: u16 = 30;
pub const FCRED: u16 = 31;
pub const FCGREEN: u16 = 32;
pub const FCBROWN: u16 = 33;
pub const FCBLUE: u16 = 34;
pub const FCMAGENTA: u16 = 35;
pub const FCCYAN: u16 = 36;
pub const FCLIGHTGRAY: u16 = 37;
pub const FCRICHCOLORS: u16 = 38; // requires additional parameters(s)
pub const FCDEFAULT: u16 = 39;
pub const FCDARKGRAY: u16 = 90;
pub const FCLIGHTRED: u16 = 91;
pub const FCLIGHTGREEN: u16 = 92;
pub const FCYELLOW: u16 = 93;
pub const FCLIGHTBLUE: u16 = 94;
pub const FCLIGHTMAGENTA: u16 = 95;
pub const FCLIGHTCYAN: u16 = 96;
pub const FCWHITE: u16 = 97;

// background colors
pub const BCBLACK: u16 = 40;
pub const BCRED: u16 = 41;
pub const BCGREEN: u16 = 42;
pub const BCBROWN: u16 = 43;
pub const BCBLUE: u16 = 44;
pub const BCMAGENTA: u16 = 45;
pub const BCCYAN: u16 = 46;
pub const BCLIGHTGRAY: u16 = 47;
pub const BCRICHCOLORS: u16 = 48; // requires additional parameter(s)
pub const BCDEFAULT: u16 = 49;
pub const BCDARKGRAY: u16 = 100;
pub const BCLIGHTRED: u16 = 101;
pub const BCLIGHTGREEN: u16 = 102;
pub const BCYELLOW: u16 = 103;
pub const BCLIGHTBLUE: u16 = 104;
pub const BCLIGHTMAGENTA: u16 = 105;
pub const BCLIGHTCYAN: u16 = 106;
pub const BCWHITE: u16 = 107;

#[derive(Debug, Default)]
pub struct AnsiEscapeStream {
    buffer: Cursor<Vec<u8>>,
}

impl AnsiEscapeStream {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Cursor::new(Vec::<u8>::with_capacity(capacity)),
        }
    }

    pub fn reset(&mut self) {
        self.buffer.set_position(0);
    }

    pub fn reset_attribute(&mut self, attr: u16) -> io::Result<usize> {
        let written = match attr {
            30..=37 | 90..=97 => self.write_attribute(FCDEFAULT)?,
            40..=47 | 100..=107 => self.write_attribute(BCDEFAULT)?,
            _ => panic!("code not implemented"),
        };

        Ok(written)
    }

    pub fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.buffer.write(buffer)
    }

    pub fn write_attribute(&mut self, attr: u16) -> io::Result<usize> {
        let written = self.buffer.write(&[ESC])?;
        write!(self.buffer, "[{attr}m")?;
        Ok(written)
    }

    pub fn write_string(&mut self, text: &str) -> io::Result<usize> {
        self.buffer.write(text.as_bytes())
    }

    pub fn write_text_fc(&mut self, color: u16, text: &str) -> io::Result<usize> {
        let written = match color {
            40..=47 | 100..=107 => {
                let mut w = self.write_attribute(color - 10)?;
                if !text.is_empty() {
                    w += self.write_string(text)?;
                    w += self.reset_attribute(color - 10)?;
                }
                w
            }
            _ => {
                let mut w = self.write_attribute(color)?;
                if !text.is_empty() {
                    w += self.write_string(text)?;
                    w += self.reset_attribute(color)?;
                }
                w
            }
        };

        Ok(written)
    }

    pub fn write_text_bc(&mut self, color: u16, text: &str) -> io::Result<usize> {
        let written = match color {
            30..=37 | 90..=97 => {
                let mut w = self.write_attribute(color + 10)?;
                if !text.is_empty() {
                    w += self.write_string(text)?;
                    w += self.reset_attribute(color + 10)?;
                }
                w
            }
            _ => {
                let mut w = self.write_attribute(color)?;
                if !text.is_empty() {
                    w += self.write_string(text)?;
                    w += self.reset_attribute(color)?;
                }
                w
            }
        };

        Ok(written)
    }
}

impl Deref for AnsiEscapeStream {
    type Target = Cursor<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for AnsiEscapeStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl<T> AsMut<T> for AnsiEscapeStream
where
    <AnsiEscapeStream as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl<T> AsRef<T> for AnsiEscapeStream
where
    T: ?Sized,
    <AnsiEscapeStream as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_bytes() {
        // write bytes to stream using byte string literal aka. b.
        let mut astream = AnsiEscapeStream::new(5);
        astream.write(b"abcde").unwrap();

        let result = String::from_utf8_lossy(astream.buffer.get_ref());
        assert_eq!("abcde", result);

        // reset stream
        astream.reset();

        // write byes to stream using an array
        let arr = [1, 2, 3, 4, 5];
        astream.write(&arr).unwrap();
        let vec = astream.buffer.get_ref();
        assert_eq!(&arr, vec.as_slice());

        // reset stream
        astream.reset();

        // write a vector to stream
        let vec = vec![1, 2, 3, 4, 5];
        astream.write(&vec).unwrap();
        assert_eq!(&vec, astream.buffer.get_ref());
    }

    #[test]
    fn test_write_attribute_function() {
        let mut astream = AnsiEscapeStream::default();
        astream.write_attribute(FCRED).unwrap();
        // fcred escape code
        let vec = astream.get_ref();
        assert_eq!(&[0x1b, 0x5b, 0x33, 0x31, 0x6d], vec.as_slice());
    }

    #[test]
    fn test_reset_attibute_function() {
        let mut astream = AnsiEscapeStream::default();
        astream.reset_attribute(FCRED).unwrap();
        // fcred escape code
        let vec = astream.get_ref();
        assert_eq!(&[0x1b, 0x5b, 0x33, 0x39, 0x6d], vec.as_slice());
    }

    #[test]
    fn test_drefmut_implementation() {
        let mut astream = AnsiEscapeStream::default();
        astream.write_attribute(FCRED).unwrap();
        let mut output = Cursor::new(Vec::<u8>::new());
        astream.reset();
        let buffer: &mut Cursor<Vec<u8>> = &mut astream;
        std::io::copy(buffer, &mut output).unwrap();
        assert_eq!(&[0x1b, 0x5b, 0x33, 0x31, 0x6d], output.get_ref().as_slice());
    }

    #[test]
    fn test_write_text_fc_function() {
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_fc(FCGREEN, "123").unwrap();
        // asserts that fcred was writed and also reseted with fcdefault
        assert_eq!(
            &[0x1b, 0x5b, 0x33, 0x32, 0x6d, 0x31, 0x32, 0x33, 0x1b, 0x5b, 0x33, 0x39, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();
        astream.get_mut().clear();

        // asserts that without text argument write_text_color dont resets format
        astream.write_text_fc(FCYELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x39, 0x33, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();

        // asserts that if we use a background color, write_text_color will convert it to foreground color
        astream.write_text_fc(BCYELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x39, 0x33, 0x6d],
            astream.get_ref().as_slice()
        )
    }

    #[test]
    fn test_write_text_bc_function() {
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_bc(BCGREEN, "123").unwrap();
        // asserts that bcgreen was writed and also reseted with bcdefault
        assert_eq!(
            &[0x1b, 0x5b, 0x34, 0x32, 0x6d, 0x31, 0x32, 0x33, 0x1b, 0x5b, 0x34, 0x39, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();
        astream.get_mut().clear();

        // asserts that without text argument write_text_bc dont resets format
        astream.write_text_bc(BCYELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x31, 0x30, 0x33, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();

        // asserts that if we use a foreground color, write_text_bc will convert it to background color
        astream.write_text_bc(FCYELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x31, 0x30, 0x33, 0x6d],
            astream.get_ref().as_slice()
        )       
    }
}
