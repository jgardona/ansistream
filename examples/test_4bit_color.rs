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

    astream.write_string("Writing 4bit color to stdout\n")?;

    astream.write_text_fc(FCBLACK, "foreground black text color\n")?;
    astream.write_text_fc(FCRED, "foreground red text color\n")?;
    astream.write_text_fc(FCGREEN, "foreground green text color\n")?;
    astream.write_text_fc(FCBROWN, "foreground brown text color\n")?;
    astream.write_text_fc(FCBLUE, "foreground blue text color\n")?;
    astream.write_text_fc(FCMAGENTA, "foreground magenta text color\n")?;
    astream.write_text_fc(FCCYAN, "foreground cyan text color\n")?;
    astream.write_text_fc(FCDARKGRAY, "foreground dark gray text color\n")?;
    astream.write_text_fc(FCLIGHTGRAY, "foreground light gray text color\n")?;
    astream.write_text_fc(FCLIGHTRED, "foreground light red text color\n")?;
    astream.write_text_fc(FCLIGHTGREEN, "foreground light green text color\n")?;
    astream.write_text_fc(FCYELLOW, "foreground yellow text color\n")?;
    astream.write_text_fc(FCLIGHTBLUE, "foreground light blue text color\n")?;
    astream.write_text_fc(FCLIGHTMAGENTA, "foreground light magenta text color\n")?;
    astream.write_text_fc(FCLIGHTCYAN, "foreground light cyan text color\n")?;
    astream.write_text_fc(FCWHITE, "foreground white text color\n")?;

    astream.write_string(
        "\nif we write a background color in write_text_fc, it will convert to foreground color\n",
    )?;

    astream.write_text_fc(BCRED, "converted to foreground red color\n\n")?;
    astream.write_text_fc(
        FCCYAN,
        "also, if we write a text with non empty string, write_text_fc, will reset the escape code",
    )?;
    astream.write_string("\nnormal text without escape codes\n\n")?;
    astream.write_text_fc(FCYELLOW, "")?;
    astream.write_string(
        "and if the text is empty, will not reset the scape code(check in code line above)\n\n",
    )?;

    astream.write_text_bc(BCMAGENTA, "the same logics is valid for background color")?;
    astream.write_string("\ntext escape code was reseted\n\n")?;
    astream.write_text_fc(FCCYAN, "")?;
    astream.write_text_bc(FCMAGENTA, "we can also set foreground with write_text_fc and omit the text, and call write_text_bc with text to use the both features")?;
    astream.write_string("\nafter that the escape code is reseted for the background\n\n")?;

    astream.reset_attribute(FCMAGENTA)?;
    astream.write_string("reset_attribute function will reset an attribute type, for example, if you pass an foreground attribute, it will reset the stream to fcdefault\n\n")?;

    astream.write_string("printing background colors\n\n")?;
    astream.write_text_fc(FCBLACK, "")?;

    for i in 40..=47 {
        astream.write_text_color(FCLIGHTGRAY, i, &format!("   {i}"))?;
    }
    writeln!(&mut *astream)?;
    for i in 100..=107 {
        astream.write_text_color(FCLIGHTGRAY, i, &format!("  {i}"))?;
    }

    let mut stdout = std::io::stdout().lock();
    astream.reset();

    flush(&mut *astream, &mut stdout)?;

    Ok(())
}