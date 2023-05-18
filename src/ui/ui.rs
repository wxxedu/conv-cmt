pub trait UI {
    /// Writes the data to the user.
    fn write(&mut self, data: impl AsRef<str>);

    /// Writes the line to the user.
    fn writeln(&mut self, data: impl AsRef<str>);

    /// Reads the data from the user.
    fn read(&mut self, data: &mut [u8]);

    /// Selects a single item from a list of items.
    fn select(&mut self, data: &[(bool, impl AsRef<str>)]) -> usize;

    /// Selects multiple items from a list of items.
    fn multi_select(
        &mut self,
        data: &[(bool, impl AsRef<str>)],
        out: &mut [bool],
    );

    /// Edits the text in the editor.
    fn edit(&mut self, initial_data: impl AsRef<str>) -> String;
}
