use std::io;
use std::io::Write;

use ansistream::FC_DARK_GRAY;

fn hex2rgb(hex: u32) -> (u16, u16, u16) {
    let r = (hex >> 16) as u16;
    let g = ((hex >> 8) & 0xff) as u16;
    let b = (hex & 0xff) as u16;

    (r, g, b)
}

fn main() -> io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(io::stdout().lock());

    let palettes: [u32; 50] = [
        0xf1c15d, 0x85a746, 0x599a70, 0xe56a4b, 0xeda052, 0xb3d9e2, 0xb3b4ac, 0xfefacb, 0x78495d,
        0x93667a, 0x36473d, 0x778c63, 0xe8df7a, 0xe6a91b, 0xea9804, 0x242623, 0x7d603e, 0xdfd1a4,
        0x5e7e4f, 0xc74223, 0x4b9d8e, 0x85cd9f, 0xa4deab, 0xc9e8c9, 0xfff7c9, 0xfff7c9, 0xfff7c9,
        0xffca3d, 0xffffff, 0xb80000, 0x8f3631, 0xb9512f, 0xc96c36, 0xaa883f, 0x3b4648, 0xfeed01,
        0xff9b0d, 0xff4732, 0xfc3f5c, 0x935ca4, 0xe2e1e9, 0xcdc9d0, 0xa5aabe, 0x5f6273, 0xf00806,
        0x9e3740, 0xf3d99e, 0xb1b88c, 0x55887f, 0x2b2e37,
    ];

    astream.write_string("Printing a common brazilian color palette:\n\n")?;

    for (idx, &c) in palettes.iter().enumerate() {
        if idx % 5 == 0 {
            astream.write_string("\t")?;
        }

        if idx % 10 == 0 {
            writeln!(&mut *astream)?;
        }
        astream.write_attribute(FC_DARK_GRAY)?;
        let (r, g, b) = hex2rgb(c);
        astream.write_text_bcrgb_fmt(r, g, b, format_args!("   {c:#06x} "))?;
    }

    astream.reset_all_attributes()?;

    Ok(())
}
