export function submit(config, configInit, useridField, usernameField) {
    if (configInit) {
        config.configId = useridField.text.trim();
    }
    config.name = usernameField.text.trim();
}
