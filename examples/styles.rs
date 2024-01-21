use std::io::{self, Write};

use ansistream::{
    TS_BLINK, TS_BOLD, TS_DIM, TS_HIDDEN, TS_INVERT, TS_ITALIC, TS_OVERLINE, TS_STRIKE,
    TS_UNDERLINE,
};

fn main() -> io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(std::io::stdout().lock());
    astream.write_string(
        "This example will print text styles. Will depend on terminal emulator specs:\n\n",
    )?;
    astream.write_attribute(TS_BOLD)?;
    astream.write_string("This is a bold styled text\n")?;
    astream.reset_all_attributes()?;

    astream.write_attribute(TS_ITALIC)?;
    astream.write_string("This is a italic styled text\n")?;
    astream.reset_all_attributes()?;

    astream.write_attribute(TS_DIM)?;
    astream.write_string("This is a dim styled text\n")?;
    astream.reset_all_attributes()?;

    astream.write_attribute(TS_INVERT)?;
    astream.write_string("This is a invert styled text")?;
    astream.reset_all_attributes()?;
    writeln!(&mut *astream)?;
    writeln!(&mut *astream)?;

    astream.write_attribute(TS_OVERLINE)?;
    astream.write_string("This is a overline styled text\n")?;
    astream.reset_all_attributes()?;

    astream.write_attribute(TS_UNDERLINE)?;
    astream.write_string("This is a underline styled text\n")?;
    astream.reset_all_attributes()?;

    astream.write_attribute(TS_HIDDEN)?;
    astream.write_string("This is a hidden styled text\n")?;
    astream.reset_all_attributes()?;

    astream.write_attribute(TS_BLINK)?;
    astream.write_string("This is a blink styled text\n")?;
    astream.reset_all_attributes()?;

    astream.write_attribute(TS_STRIKE)?;
    astream.write_string("This is a strike styled text\n")?;
    astream.reset_all_attributes()?;

    Ok(())
}
