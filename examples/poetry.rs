use std::{
    fs,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(std::io::stdout().lock());
    let text = fs::read_to_string("tests/data/vmpoetry.txt")?;
    text.chars().for_each(|c| {
        if c.is_alphabetic() {
            astream
                .write_text_fc256_fmt(((c as u16) % 19) + 150, format_args!("{c}"))
                .unwrap();
        } else {
            write!(*astream, "{c}").unwrap();
        }
    });
    Ok(())
}
