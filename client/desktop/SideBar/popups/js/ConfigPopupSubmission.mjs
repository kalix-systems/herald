export function submit(config, usernameField) {
    const newName = usernameField.text.trim();
    config.name = newName;
}
