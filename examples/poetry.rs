use std::{
    fs,
    io::{self, Read, Seek, Write},
};

fn flush_stdout<W: Read + Seek>(reader: &mut W) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    reader.seek(io::SeekFrom::Start(0))?;
    io::copy(reader, &mut stdout)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::new(2000);
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
    flush_stdout(&mut *astream)?;
    Ok(())
}
