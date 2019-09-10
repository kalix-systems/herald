export function safePfpUrl(config: Config): string {
  if (config.pfpUrl === null) {
    return "";
  } else {
    return config.pfpUrl;
  }
}
