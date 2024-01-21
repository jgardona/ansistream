use std::io::Write;

use ansistream::*;

fn main() -> std::io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(std::io::stdout().lock());

    astream.write_string("Printing 16 color to stdout\n\n")?;

    for i in 40..=47 {
        astream.write_text_color_fmt(FC_LIGHT_GRAY, i, format_args!("{i:>5} "))?;
    }
    writeln!(&mut *astream)?;
    for i in 100..=107 {
        astream.write_text_color_fmt(FC_LIGHT_GRAY, i, format_args!("{i:>5} "))?;
    }

    Ok(())
}
