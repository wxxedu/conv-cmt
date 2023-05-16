pub fn ask_push() -> bool {
    dialoguer::Confirm::new()
        .with_prompt("Push to remote?")
        .interact()
        .unwrap()
}
