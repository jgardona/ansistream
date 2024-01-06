use ansistream::{BCBLACK, BCBLUE, BCBROWN, BCCYAN, BCGREEN, BCMAGENTA, BCRED, FCDARKGRAY};

fn main() -> std::io::Result<()> {
    let mut astream = ansistream::AnsiEscapeStream::default();
    let mut stdout = std::io::stdout().lock();
    for _ in 0..1000 {
        astream.write_text_fc(FCDARKGRAY, "")?;
        astream.write_text_bc(BCBLACK, "  ")?;
        astream.write_text_bc(BCRED, "  ")?;
        astream.write_text_bc(BCGREEN, "  ")?;
        astream.write_text_bc(BCBROWN, "  ")?;
        astream.write_text_bc(BCBLUE, "  ")?;
        astream.write_text_bc(BCMAGENTA, "  ")?;
        astream.write_text_bc(BCCYAN, "  ")?;
        astream.reset();
        std::io::copy(&mut *astream, &mut stdout)?;
        astream.reset();
    }

    Ok(())
}
