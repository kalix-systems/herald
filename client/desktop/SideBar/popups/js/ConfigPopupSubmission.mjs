export function submit(config, usernameField) {
    const newName = usernameField.text.trim();
    if (newName === "") {
        return;
    }
    config.name = newName;
}
