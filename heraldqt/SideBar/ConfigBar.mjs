export function safePfpUrl(config) {
    if (config.pfpUrl === null) {
        return "";
    }
    else {
        return config.pfpUrl;
    }
}
