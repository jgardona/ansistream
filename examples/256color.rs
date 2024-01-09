use std::{
    io::Write,
    io::{Read, Seek},
};

fn flush<R: Read + Seek, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<()> {
    reader.seek(std::io::SeekFrom::Start(0))?;
    std::io::copy(reader, writer)?;
    reader.seek(std::io::SeekFrom::Start(0))?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::default();
    let mut stdout = std::io::stdout().lock();

    astream.write_string("Printing 256 color(16 bit) table\n\n")?;
    astream.write_string("Printing standard and extended colors:\n\n")?;

    for i in 0..=7 {
        astream.write_text_bc256_fmt(i, format_args!("{i:<4}"))?;
    }

    astream.write_string("   ")?;

    for i in 0..=7 {
        let ie = i + 8;
        astream.write_text_bc256_fmt(ie, format_args!("{ie:<4}"))?;
    }

    astream.reset_all_attributes()?;
    astream.write_string("\n\nPrinting 256 color:\n\n")?;

    let mut fg = 0;
    for i in 0..=215 {
        let v = i + 16;
        if i != 0 {
            if i % 18 == 0 {
                fg = 0;
                astream.reset_all_attributes()?;
                writeln!(&mut *astream)?;
            }
            if i % 36 == 0 {
                fg = 255;
            }
        }
        astream.write_attribute(fg)?;
        astream.write_text_bc256_fmt(v, format_args!("{v:<4}"))?;
    }

    astream.reset_all_attributes()?;
    astream.write_string("\n\nPrinting gray color:\n\n")?;

    for i in 0..=23 {
        if i < 12 {
            fg = 255;
        } else {
            fg = 0;
        }

        let ie = i + 232;
        astream.write_attribute(fg)?;
        astream.write_text_bc256_fmt(ie, format_args!("{ie:<4}"))?;
    }

    flush(&mut *astream, &mut stdout)?;

    Ok(())
}
