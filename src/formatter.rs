use std::io;

#[derive(Clone, Debug)]
pub struct NewLineFormatter {
    depth: usize,
}

impl NewLineFormatter {
    pub(super) fn new() -> Self {
        NewLineFormatter { depth: 0 }
    }
}

impl serde_json::ser::Formatter for NewLineFormatter {
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.depth += 1;
        writer.write_all(b"{")
    }

    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.depth -= 1;

        writer.write_all(b"}")?;
        if self.depth == 0 {
            writer.write_all(b"\n")?;
        }

        Ok(())
    }
}
