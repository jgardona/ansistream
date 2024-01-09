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

/// Text Styles
pub const TS_RESET_ALL: u16 = 0;
pub const TS_BOLD: u16 = 1;
pub const TS_DIM: u16 = 2;
pub const TS_ITALIC: u16 = 3;
pub const TS_UNDERLINE: u16 = 4;
pub const TS_BLINK: u16 = 5;
pub const TS_OVERLINE: u16 = 6;
pub const TS_INVERT: u16 = 7;
pub const TS_HIDDEN: u16 = 8;
pub const TS_STRIKE: u16 = 9;
pub const TS_DEFAULT: u16 = 20;
pub const TS_NO_BOLD: u16 = 21;
pub const TS_NO_DIM: u16 = 22;
pub const TS_NO_ITALIC: u16 = 0; // 23; not working
pub const TS_NO_UNDERLINE: u16 = 24;
pub const TS_NO_BLINK: u16 = 25;
pub const TS_NO_OVERLINE: u16 = 26;
pub const TS_NO_INVERT: u16 = 27;
pub const TS_NO_HIDDEN: u16 = 28;
pub const TS_NO_STRIKE: u16 = 29;
/// Foreground colors.
pub const FC_BLACK: u16 = 30;
pub const FC_RED: u16 = 31;
pub const FC_GREEN: u16 = 32;
pub const FC_BROWN: u16 = 33;
pub const FC_BLUE: u16 = 34;
pub const FC_MAGENTA: u16 = 35;
pub const FC_CYAN: u16 = 36;
pub const FC_LIGHT_GRAY: u16 = 37;
pub const FC_RICH_COLORS: u16 = 38; // requires additional parameters(s)
pub const FC_DEFAULT: u16 = 39;
pub const FC_DARK_GRAY: u16 = 90;
pub const FC_LIGHT_RED: u16 = 91;
pub const FC_LIGHT_GREEN: u16 = 92;
pub const FC_YELLOW: u16 = 93;
pub const FC_LIGHT_BLUE: u16 = 94;
pub const FC_LIGHT_MAGENTA: u16 = 95;
pub const FC_LIGHT_CYAN: u16 = 96;
pub const FC_WHITE: u16 = 97;

/// Background colors.
pub const BC_BLACK: u16 = 40;
pub const BC_RED: u16 = 41;
pub const BC_GREEN: u16 = 42;
pub const BC_BROWN: u16 = 43;
pub const BC_BLUE: u16 = 44;
pub const BC_MAGENTA: u16 = 45;
pub const BC_CYAN: u16 = 46;
pub const BC_LIGHT_GRAY: u16 = 47;
pub const BC_RICH_COLORS: u16 = 48; // requires additional parameter(s)
pub const BC_DEFAULT: u16 = 49;
pub const BC_DARK_GRAY: u16 = 100;
pub const BC_LIGHT_RED: u16 = 101;
pub const BC_LIGHT_GREEN: u16 = 102;
pub const BC_YELLOW: u16 = 103;
pub const BC_LIGHT_BLUE: u16 = 104;
pub const BC_LIGHT_MAGENTA: u16 = 105;
pub const BC_LIGHT_CYAN: u16 = 106;
pub const BC_WHITE: u16 = 107;

/// Data structure used to do fast ansi escape write operations.
/// It implements many methods and traits which makes easier to format text.
/// An internal buffer can be preallocated, which avoids allocation using write operations.
#[derive(Debug, Default)]
pub struct AnsiEscapeStream {
    buffer: Cursor<Vec<u8>>,
}

impl AnsiEscapeStream {
    /// Initializes an AnsiEscapeStream.\
    /// capacity is a unsigned number used to preallocate the internal buffer.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Cursor::new(Vec::<u8>::with_capacity(capacity)),
        }
    }

    /// Clear the internal buffer.\
    /// The buffer position is updated to 0, and all data is cleared. The capacity remains the same.
    pub fn clear(&mut self) {
        self.buffer.set_position(0);
        self.buffer.get_mut().clear();
    }

    /// Set the internal buffer position to 0.
    pub fn reset(&mut self) {
        self.buffer.set_position(0);
    }

    /// Reset all ansi escape code attributes before this buffer position using ESC[0m.
    pub fn reset_all_attributes(&mut self) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[0m")?;
        Ok(())
    }

    /// Reset an attribute of type. If the attribute is a foreground color, then
    /// reset it to the default foreground color.
    pub fn reset_attribute(&mut self, attr: u16) -> io::Result<()> {
        match attr {
            TS_BOLD => self.write_attribute(TS_NO_BOLD)?,
            TS_DIM => self.write_attribute(TS_NO_DIM)?,
            TS_ITALIC => self.write_attribute(TS_NO_ITALIC)?,
            TS_UNDERLINE => self.write_attribute(TS_NO_UNDERLINE)?,
            TS_BLINK => self.write_attribute(TS_NO_BLINK)?,
            TS_OVERLINE => self.write_attribute(TS_NO_OVERLINE)?,
            TS_INVERT => self.write_attribute(TS_NO_INVERT)?,
            TS_HIDDEN => self.write_attribute(TS_NO_HIDDEN)?,
            TS_STRIKE => self.write_attribute(TS_NO_STRIKE)?,
            30..=37 | 90..=97 => self.write_attribute(FC_DEFAULT)?,
            40..=47 | 100..=107 => self.write_attribute(BC_DEFAULT)?,
            _ => panic!("code not implemented"),
        };

        Ok(())
    }

    /// Write a byte slice to stream.
    pub fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.buffer.write(buffer)
    }

    /// Write an attribute to stream.
    pub fn write_attribute(&mut self, attr: u16) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{attr}m")?;
        Ok(())
    }

    /// Write a string to stream.
    pub fn write_string(&mut self, text: &str) -> io::Result<usize> {
        self.buffer.write(text.as_bytes())
    }

    /// Write a 16 foreground color text to stream. The attribute is reseted in the end of the text.
    /// If the text is empty, the reset operation will not be performed.
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

    /// Write a 16 background color text to stream. The attribute is reseted in the end of the text.
    /// If the text is empty, the reset operation will not be performed.
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

    /// Write 16 foregournd and background color text to stream. If the text is
    /// empty, the reset operation will not be performed.
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

    /// Write 256 foreground color text to the stream. If the text is empty, the
    /// reset operation will not be performed.
    pub fn write_text_fc256(&mut self, color: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{FC_RICH_COLORS};5;{color}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(color)?;
        }
        Ok(())
    }

    /// Write 256 background color text to the stream. If the text is empty, the
    /// reset operation will not be performed.
    pub fn write_text_bc256(&mut self, color: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{BC_RICH_COLORS};5;{color}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(color)?;
        }
        Ok(())
    }

    /// Write RGB foreground color text to the stream. If the text is empty, the
    /// reset operation will not be performed.
    pub fn write_text_fcrgb(&mut self, r: u16, g: u16, b: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{FC_RICH_COLORS};2;{r};{g};{b}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(FC_BLACK)?;
        }
        Ok(())
    }

    /// Write RGB background color text to the stream. If the text is empty, the
    /// reset operation will not be performed.
    pub fn write_text_bcrgb(&mut self, r: u16, g: u16, b: u16, text: &str) -> io::Result<()> {
        self.buffer.write_all(&[ESC])?;
        write!(self.buffer, "[{BC_RICH_COLORS};2;{r};{g};{b}m{text}")?;
        if !text.is_empty() {
            self.reset_attribute(BC_BLACK)?;
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
        astream.write_attribute(FC_RED).unwrap();
        // fcred escape code
        let vec = astream.get_ref();
        assert_eq!(&[0x1b, 0x5b, 0x33, 0x31, 0x6d], vec.as_slice());
    }

    #[test]
    fn test_reset_attibute_function() {
        let mut astream = AnsiEscapeStream::default();
        astream.reset_attribute(FC_RED).unwrap();
        // fcred escape code
        let vec = astream.get_ref();
        assert_eq!(&[0x1b, 0x5b, 0x33, 0x39, 0x6d], vec.as_slice());
    }

    #[test]
    fn test_drefmut_implementation() {
        let mut astream = AnsiEscapeStream::default();
        astream.write_attribute(FC_RED).unwrap();
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
        astream.write_text_fc(FC_GREEN, "123").unwrap();
        // asserts that fcred was writed and also reseted with fcdefault
        assert_eq!(
            &[0x1b, 0x5b, 0x33, 0x32, 0x6d, 0x31, 0x32, 0x33, 0x1b, 0x5b, 0x33, 0x39, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();
        astream.get_mut().clear();

        // asserts that without text argument write_text_color dont resets format
        astream.write_text_fc(FC_YELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x39, 0x33, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();

        // asserts that if we use a background color, write_text_color will convert it to foreground color
        astream.write_text_fc(BC_YELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x39, 0x33, 0x6d],
            astream.get_ref().as_slice()
        )
    }

    #[test]
    fn test_write_text_bc_function() {
        let mut astream = AnsiEscapeStream::default();
        astream.write_text_bc(BC_GREEN, "123").unwrap();
        // asserts that bcgreen was writed and also reseted with bcdefault
        assert_eq!(
            &[0x1b, 0x5b, 0x34, 0x32, 0x6d, 0x31, 0x32, 0x33, 0x1b, 0x5b, 0x34, 0x39, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();
        astream.get_mut().clear();

        // asserts that without text argument write_text_bc dont resets format
        astream.write_text_bc(BC_YELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x31, 0x30, 0x33, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();

        // asserts that if we use a foreground color, write_text_bc will convert it to background color
        astream.write_text_bc(FC_YELLOW, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x31, 0x30, 0x33, 0x6d],
            astream.get_ref().as_slice()
        )
    }

    #[test]
    fn test_write_text_color_function() {
        // test not reset scenario
        let mut astream = AnsiEscapeStream::default();
        astream
            .write_text_color(FC_MAGENTA, BC_DARK_GRAY, "")
            .unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x33, 0x35, 0x3b, 0x31, 0x30, 0x30, 0x6d],
            astream.get_ref().as_slice()
        );

        astream.reset();
        // test reset all scenario
        astream
            .write_text_color(FC_MAGENTA, BC_DARK_GRAY, "012")
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
        astream.write_text_fc256(FC_BLUE, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x33, 0x38, 0x3b, 0x35, 0x3b, 0x33, 0x34, 0x6d],
            astream.get_ref().as_slice()
        );
        astream.reset();

        // test reseting scenario
        astream.write_text_fc256(FC_BLUE, "012").unwrap();
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
        astream.write_text_bc256(BC_BLUE, "").unwrap();
        assert_eq!(
            &[0x1b, 0x5b, 0x34, 0x38, 0x3b, 0x35, 0x3b, 0x34, 0x34, 0x6d],
            astream.get_ref().as_slice()
        );
        astream.reset();

        // test reseting scenario
        astream.write_text_bc256(BC_BLUE, "012").unwrap();
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
