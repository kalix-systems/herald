export function submit(
  config: Config,
  useridField: TextField,
  usernameField: TextField
): void {
  if (!config.exists()) {
    config.configId = useridField.text.trim();
  }
  config.name = usernameField.text.trim();
}
