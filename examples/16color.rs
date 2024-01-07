use std::io::{Read, Seek, SeekFrom, Write};

use ansistream::*;

fn flush<T: Read + Seek, W: Write>(reader: &mut T, writer: &mut W) -> std::io::Result<()> {
    reader.seek(SeekFrom::Start(0))?;
    std::io::copy(reader, writer)?;
    reader.seek(SeekFrom::Start(0))?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::default();

    astream.write_string("Printing 16 color to stdout\n\n")?;

    astream.write_text_fc(FCBLACK, "")?;
    for i in 40..=47 {
        astream.write_text_color(FCLIGHTGRAY, i, &format!("   {i}"))?;
    }
    writeln!(&mut *astream)?;
    for i in 100..=107 {
        astream.write_text_color(FCLIGHTGRAY, i, &format!("  {i}"))?;
    }

    let mut stdout = std::io::stdout().lock();

    flush(&mut *astream, &mut stdout)?;

    Ok(())
}
