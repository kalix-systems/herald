export function submit(config: Config, usernameField: TextField): void {
  const newName = usernameField.text.trim();

  if (newName === "") {
    return;
  }

  config.name = newName;
}
