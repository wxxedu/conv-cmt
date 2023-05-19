use std::io::Write;

/// A trait for writing the content to a `Write` object. The benefit of having
/// this trait is that we can do some string manipulation without having to
/// allocate a new string.
pub trait Writer {
    /// Writes the content of the object to the given writer.
    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;

    /// Same as `write_to`, but appends a newline to the end.
    fn write_line_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.write_to(writer)?;
        writer.write_all(b"\n")?;
        Ok(())
    }

    /// Same as `write_to`, but includes a flush operation.
    fn print_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.write_to(writer)?;
        Ok(())
    }

    /// Same as `print_to`, but appends a newline to the end.
    ///
    /// Compared with the `write_line_to` method, this method will include the
    /// flush operation.
    fn print_line_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.write_line_to(writer)?;
        writer.flush()?;
        Ok(())
    }
}

impl<T: AsRef<str>> Writer for T {
    fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.as_ref().as_bytes())?;
        Ok(())
    }
}
