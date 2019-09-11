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
export function avatarShape(shape, avatar) {
    switch (shape) {
        case 0 /* Circle */:
            return avatar.size;
        default:
            throw new Error(String(shape) + " is not a shape variant");
    }
}
