//! # Ansi Stream
//! Write blazingly fast, free allocation ansi escape codes to a buffer, and flushes them all to any output
//! stream. Supports 8/16 colors, 256 colors, RGB color rendering output.
//!
//! ## ANSI Escape Codes for Terminal Graphics
//! The ANSI escape code standard, formally adopted as ISO/IEC 6429, defines a series of control sequences.
//! Each control sequence begins with a **Control Sequence Introducer** (CSI), defined as a scape character
//! followed immediately by a bracket: **ESC[**. In particular, a CSI followed by a certain number of "parameter bytes"
//! (ASCII 0-9:; <=>?) then the letter m forms a control sequence known as **Select Graphic Rendition** (SGR). If no
//! parameter bytes are explicitly given, then it is assumed to be 0. SGR parameters can be chained together with a semicolon **;**
//! as **delimiter**.
//!
//! Some common SGR parameters are shown below.
//!
//! |Parameter | Effect|
//! |- | -|
//! |0 | reset all SGR effects to their default|
//! |1 | bold or increased intensity|
//! |2 | faint or decreased insensity|
//! |4 | singly underlined|
//! |5 | slow blink|
//! |30-37 | foreground color (3/4 bit)|
//! |38;5;x | foreground color (256 colors, non-standard)|
//! |38;2;r;g;b | foreground color (RGB, non-standard)|
//! |40-47 | background color (8 colors)|
//! |48;5;x | background color (256 colors, non-standard)|
//! |48;2;r;g;b | background color (RGB, non-standard)|
//! |90-97 | bright foreground color (non-standard)|
//! |100-107 | bright background color (non-standard)|

use std::{
    io::{self, Cursor, Write},
    ops::{Deref, DerefMut},
};

/// ASCII Escape.
const ESC: u8 = 0x1b;
/// Foreground colors.
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

/// Background colors.
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

    pub fn clear(&mut self) {
        self.buffer.set_position(0);
        self.buffer.get_mut().clear();
    }

    pub fn reset(&mut self) {
        self.buffer.set_position(0);
    }

    pub fn reset_all_attributes(&mut self) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[0m")?;
        Ok(())
    }

    pub fn reset_attribute(&mut self, attr: u16) -> io::Result<()> {
        match attr {
            30..=37 | 90..=97 => self.write_attribute(FCDEFAULT)?,
            40..=47 | 100..=107 => self.write_attribute(BCDEFAULT)?,
            _ => panic!("code not implemented"),
        };

        Ok(())
    }

    pub fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.buffer.write(buffer)
    }

    pub fn write_attribute(&mut self, attr: u16) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{attr}m")?;
        Ok(())
    }

    pub fn write_string(&mut self, text: &str) -> io::Result<usize> {
        self.buffer.write(text.as_bytes())
    }

    pub fn write_text_fc(&mut self, color: u16, text: &str) -> io::Result<()> {
        match color {
            40..=47 | 100..=107 => {
                self.write_attribute(color - 10)?;
                if !text.is_empty() {
                    self.write_string(text)?;
                    self.reset_attribute(color - 10)?;
                }
            }
            _ => {
                self.write_attribute(color)?;
                if !text.is_empty() {
                    self.write_string(text)?;
                    self.reset_attribute(color)?;
                }
            }
        };

        Ok(())
    }

    pub fn write_text_bc(&mut self, color: u16, text: &str) -> io::Result<()> {
        match color {
            30..=37 | 90..=97 => {
                self.write_attribute(color + 10)?;
                if !text.is_empty() {
                    self.write_string(text)?;
                    self.reset_attribute(color + 10)?;
                }
            }
            _ => {
                self.write_attribute(color)?;
                if !text.is_empty() {
                    self.write_string(text)?;
                    self.reset_attribute(color)?;
                }
            }
        };

        Ok(())
    }

    pub fn write_text_color(
        &mut self,
        foreground: u16,
        background: u16,
        text: &str,
    ) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{foreground};{background}m{text}")?;
        if !text.is_empty() {
            self.reset_all_attributes()?;
        }
        Ok(())
    }

    pub fn write_text_fc256(&mut self, color: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{FCRICHCOLORS};5;{color}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(color)?;
        }
        Ok(())
    }

    pub fn write_text_bc256(&mut self, color: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{BCRICHCOLORS};5;{color}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(color)?;
        }
        Ok(())
    }

    pub fn write_text_fcrgb(&mut self, r: u16, g: u16, b: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{FCRICHCOLORS};2;{r};{g};{b}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(FCBLACK)?;
        }
        Ok(())
    }

    pub fn write_text_bcrgb(&mut self, r: u16, g: u16, b: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{BCRICHCOLORS};2;{r};{g};{b}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(BCBLACK)?;
        }
        Ok(())
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
        std::io::copy(&mut *astream, &mut output).unwrap();
        assert_eq!(&[0x1b, 0x5b, 0x33, 0x31, 0x6d], output.get_ref().as_slice());

        // test writing with writln! macro
        astream.clear();
        writeln!(&mut *astream).unwrap();
        assert_eq!(&[0x0a], astream.get_ref().as_slice());
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

    #[test]
    fn test_write_text_color_function() {
        // test not reset scenario
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_color(FCMAGENTA, BCDARKGRAY, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x33, 0x35, 0x3b, 0x31, 0x30, 0x30, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();
        // test reset all scenario
        astream
            .write_text_color(FCMAGENTA, BCDARKGRAY, "012")
            .unwrap();
        assert_eq!(
            &[
                0x1b, 0x5b, 0x33, 0x35, 0x3b, 0x31, 0x30, 0x30, 0x6d, 0x30, 0x31, 0x32, 0x1b, 0x5b,
                0x30, 0x6d
            ],
            astream.get_ref().as_slice()
        );
    }

    #[test]
    fn test_write_text_fc256_function() {
        // test not reseting scenario
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_fc256(FCBLUE, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x33, 0x38, 0x3b, 0x35, 0x3b, 0x33, 0x34, 0x6d],
            astream.get_ref().as_slice()
        );
        astream.reset();

        // test reseting scenario
        astream.write_text_fc256(FCBLUE, "012").unwrap();
        assert_eq!(
            &[
                0x1b, 0x5b, 0x33, 0x38, 0x3b, 0x35, 0x3b, 0x33, 0x34, 0x6d, 0x30, 0x31, 0x32, 0x1b,
                0x5b, 0x33, 0x39, 0x6d
            ],
            astream.get_ref().as_slice()
        );
    }

    #[test]
    fn test_write_text_bc256_function() {
        // test not reseting scenario
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_bc256(BCBLUE, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x34, 0x38, 0x3b, 0x35, 0x3b, 0x34, 0x34, 0x6d],
            astream.get_ref().as_slice()
        );
        astream.reset();

        // test reseting scenario
        astream.write_text_bc256(BCBLUE, "012").unwrap();
        assert_eq!(
            &[
                0x1b, 0x5b, 0x34, 0x38, 0x3b, 0x35, 0x3b, 0x34, 0x34, 0x6d, 0x30, 0x31, 0x32, 0x1b,
                0x5b, 0x34, 0x39, 0x6d
            ],
            astream.get_ref().as_slice()
        );
    }

    #[test]
    fn test_write_text_fcrgb() {
        // test not reseting scenario
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_fcrgb(255, 255, 255, "").unwrap();
        assert_eq!(
            &[
                27, 91, 0x33, 0x38, 59, 0x32, 59, 0x32, 0x35, 0x35, 59, 0x32, 0x35, 0x35, 59, 0x32,
                0x35, 0x35, 109
            ],
            astream.get_ref().as_slice()
        );
        astream.reset();

        // test reseting scenario
        astream.write_text_fcrgb(255, 255, 255, "012").unwrap();
        assert_eq!(
            &[
                27, 91, 0x33, 0x38, 59, 0x32, 59, 0x32, 0x35, 0x35, 59, 0x32, 0x35, 0x35, 59, 0x32,
                0x35, 0x35, 109, 0x30, 0x31, 0x32, 27, 91, 0x33, 0x39, 109
            ],
            astream.get_ref().as_slice()
        );
    }

    #[test]
    fn test_write_text_ccrgb() {
        // test not reseting scenario
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_bcrgb(255, 255, 255, "").unwrap();
        assert_eq!(
            &[
                27, 91, 0x34, 0x38, 59, 0x32, 59, 0x32, 0x35, 0x35, 59, 0x32, 0x35, 0x35, 59, 0x32,
                0x35, 0x35, 109
            ],
            astream.get_ref().as_slice()
        );
        astream.reset();

        // test reseting scenario
        astream.write_text_bcrgb(255, 255, 255, "012").unwrap();
        assert_eq!(
            &[
                27, 91, 0x34, 0x38, 59, 0x32, 59, 0x32, 0x35, 0x35, 59, 0x32, 0x35, 0x35, 59, 0x32,
                0x35, 0x35, 109, 0x30, 0x31, 0x32, 27, 91, 0x34, 0x39, 109
            ],
            astream.get_ref().as_slice()
        );
    }
}
