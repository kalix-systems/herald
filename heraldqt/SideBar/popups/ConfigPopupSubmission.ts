export function submit(
  config: Config,
  useridField: TextField,
  usernameField: TextField
): void {
  if (config.init) {
    config.configId = useridField.text.trim();
  }
  config.name = usernameField.text.trim();
}
