use std::io::{self, Read, Seek, Write};

use ansistream::{TS_BOLD, TS_ITALIC, TS_DIM, TS_INVERT, TS_OVERLINE, TS_UNDERLINE, TS_HIDDEN, TS_BLINK, TS_STRIKE};

fn flush<R: Read + Seek>(reader: &mut R) -> io::Result<()> {
    reader.seek(io::SeekFrom::Start(0))?;
    io::copy(reader, &mut io::stdout().lock())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(2000);
    astream.write_string("This example will print text styles. Will depend on terminal emulator specs:\n\n")?;
    astream.write_attribute(TS_BOLD)?;
    astream.write_string("This is a bold styled text\n")?;
    astream.reset_attribute(TS_BOLD)?;

    astream.write_attribute(TS_ITALIC)?; // not working today?
    astream.write_string("This is a italic styled text\n")?;
    astream.reset_attribute(TS_ITALIC)?;

    astream.write_attribute(TS_DIM)?;
    astream.write_string("This is a dim styled text\n")?;
    astream.reset_attribute(TS_DIM)?;

    astream.write_attribute(TS_INVERT)?; // not working today?
    astream.write_string("This is a invert styled text")?;
    astream.reset_attribute(TS_INVERT)?;
    writeln!(&mut *astream)?;
    writeln!(&mut *astream)?;

    astream.write_attribute(TS_OVERLINE)?; // not working today?
    astream.write_string("This is a overline styled text\n")?;
    astream.reset_attribute(TS_OVERLINE)?;

    astream.write_attribute(TS_UNDERLINE)?; // not working today?
    astream.write_string("This is a underline styled text\n")?;
    astream.reset_attribute(TS_UNDERLINE)?;

    astream.write_attribute(TS_HIDDEN)?; // not working today?
    astream.write_string("This is a hidden styled text\n")?;
    astream.reset_attribute(TS_HIDDEN)?;

    astream.write_attribute(TS_BLINK)?; // not working today?
    astream.write_string("This is a blink styled text\n")?;
    astream.reset_attribute(TS_BLINK)?;

    astream.write_attribute(TS_STRIKE)?; // not working today?
    astream.write_string("This is a strike styled text\n")?;
    astream.reset_attribute(TS_STRIKE)?;

    flush(&mut *astream)?;
    Ok(())
}