use std::io::{self, Read, Seek, Write};

fn flush_stdout<W: Read + Seek>(reader: &mut W) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    reader.seek(io::SeekFrom::Start(0))?;
    io::copy(reader, &mut stdout)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(2000);

    astream.write_string("Printing a common brazilian color palette:\n\n")?;

    astream.write_text_fcrgb(50, 50, 50, "")?;
    astream.write_text_bcrgb(139, 60, 60, " #8b3c3c          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(189, 90, 92, " #bd5a5c          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(64, 80, 103, " #405067          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(98, 112, 121, " #627079          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(156, 118, 53, " #9c7635          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(211, 189, 152, " #d3bd98          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(130, 101, 103, " #826567          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(169, 139, 139, " #a98b8b          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(255, 180, 1, " #ffb401          ")?;
    writeln!(&mut *astream)?;
    astream.write_text_bcrgb(255, 217, 115, " #ffd973          ")?;
    writeln!(&mut *astream)?;

    flush_stdout(&mut *astream)?;
    Ok(())
}
