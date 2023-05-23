pub enum TextVariant {
    UserInput,
}

pub trait UI {
    /// Prints out the given `text` to the user.
    fn print(&self, text: impl AsRef<str>, variant: TextVariant);

    /// Prints out the given `text` to the user, followed by a newline.
    fn println(&self, text: impl AsRef<str>, variant: TextVariant);

    /// Asks the user to select an option from the given `options`, with the
    /// given `default` option pre-selected.
    fn select(
        &self,
        prompt: impl AsRef<str>,
        options: &[impl AsRef<str>],
        default: Option<usize>,
    ) -> usize;

    /// Asks the user to multi-select options from the given `options`, with
    /// the given `default` options pre-selected.
    fn multi_select(
        &self,
        prompt: impl AsRef<str>,
        options: &[impl AsRef<str>],
        default: &[usize],
        on_select_one: impl FnMut(usize),
    ) -> Vec<usize>;

    /// Asks the user to input a string.
    fn input(&self) -> String;

    /// Asks the user to confirm a prompt.
    fn confirm(&self, prompt: impl AsRef<str>) -> bool;
}
