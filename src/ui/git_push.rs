use dialoguer::theme::ColorfulTheme;

pub fn ask_push() -> bool {
    dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Push to remote?")
        .interact()
        .unwrap()
}
