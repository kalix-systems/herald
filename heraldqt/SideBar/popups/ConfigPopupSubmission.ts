export function submit(
  config: Config,
  configInit: boolean,
  useridField: TextField,
  usernameField: TextField
): void {
  if (configInit) {
    config.configId = useridField.text.trim();
  }
  config.name = usernameField.text.trim();
}
