export function avatarSource(
  displayName: string,
  pfpUrl: string,
  imageAvatar: Avatar,
  initialAvatar: Avatar
): Avatar | undefined {
  if (pfpUrl === "" && displayName === "") {
    return undefined;
  } else if (pfpUrl !== "") {
    return imageAvatar;
  } else {
    return initialAvatar;
  }
}
