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

export function avatarShape(shape: AvatarShape, avatar: Avatar): number {
  switch (shape) {
    case AvatarShape.Circle:
      return avatar.size;
    default:
      throw new Error(String(shape) + " is not a shape variant");
  }
}
