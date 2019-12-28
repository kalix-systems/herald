export function submit(config: Config, usernameField: TextField): void {
  const newName = usernameField.text.trim();
  config.name = newName ;
}
