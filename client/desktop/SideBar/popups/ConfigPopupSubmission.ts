export function submit(config: Config, usernameField: TextField): void {
  config.name = usernameField.text.trim();
}
