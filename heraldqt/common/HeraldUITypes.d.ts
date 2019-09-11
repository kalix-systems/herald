declare class Avatar {
  displayName: string;
  pfpUrl: string;
}

declare enum AvatarShape {
  circle = 0
}

declare class ContactView extends ListView<ContactItem> {}

declare enum ColorScheme {
  dark = 0,
  light = 1,
  solarizedDark = 2,
  solarizedLight = 3
}
