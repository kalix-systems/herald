export function avatarSource(displayName, pfpUrl, imageAvatar, initialAvatar) {
    if (pfpUrl === "" && displayName === "") {
        return undefined;
    }
    else if (pfpUrl !== "") {
        return imageAvatar;
    }
    else {
        return initialAvatar;
    }
}
