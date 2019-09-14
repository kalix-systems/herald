export function submit(config, useridField, usernameField) {
    if (config.init) {
        config.configId = useridField.text.trim();
    }
    config.name = usernameField.text.trim();
}
