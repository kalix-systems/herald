export function submit(config, useridField, usernameField) {
    if (!config.exists()) {
        config.configId = useridField.text.trim();
    }
    config.name = usernameField.text.trim();
}
