declare class ContactView extends ListView<ContactItem> {}

declare enum ColorScheme {
  Dark = 0,
  Light = 1,
  SolarizedDark = 2,
  SolarizedLight = 3
}

declare class TextAreaForm {
  state: string;
  replyId: MsgId;
}

declare class ConversationWindow {
  contentY: number;
  height: number;
  returnToBounds(): void;
}

declare class ContactItem extends Item {
  contactAvatar: Avatar;
}

declare class Avatar {
  avatarLabel: string;
  pfpUrl: string;
  size: number;
}

declare const enum AvatarShape {
  Circle = 0
}
