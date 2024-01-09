use std::io::{self, Read, Seek, Write};

use ansistream::{FCDARKGRAY, BCBLACK};

fn flush_stdout<W: Read + Seek>(reader: &mut W) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    reader.seek(io::SeekFrom::Start(0))?;
    io::copy(reader, &mut stdout)?;
    Ok(())
}

fn hex2rgb(hex: u32) -> (u16, u16, u16) {
    let r = (hex >> 16) as u16;
    let g = ((hex >> 8) & 0xff) as u16;
    let b = (hex & 0xff) as u16;

    (r, g, b)
} 

fn main() -> io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(2000);

    let palettes: [u32; 20] = [
        0xf1c15d,
        0x85a746,
        0x599a70,
        0xe56a4b,
        0xeda052,
        0xb3d9e2,
        0xb3b4ac,
        0xfefacb,
        0x78495d,
        0x93667a,
        0x36473d,
        0x778c63,
        0xe8df7a,
        0xe6a91b,
        0xea9804,
        0x242623,
        0x7d603e,
        0xdfd1a4,
        0x5e7e4f,
        0xc74223
    ];

    astream.write_string("Printing a common brazilian color palette:\n\n")?;

    for (idx, &c) in palettes.iter().enumerate() {
        if idx % 5 == 0 {
            astream.reset_attribute(BCBLACK)?;
            astream.write_string("\t")?;
        }

        if idx % 10 == 0 {
            astream.reset_attribute(BCBLACK)?;
            writeln!(&mut *astream)?;
        }
        astream.write_text_fc(FCDARKGRAY, "")?;
        let rgb = hex2rgb(c);
        astream.write_text_bcrgb(rgb.0, rgb.1, rgb.2, "")?;
        write!(&mut *astream, "   {c:#06x}")?;
    }

    astream.reset_all_attributes()?;

    flush_stdout(&mut *astream)?;
    Ok(())
}
