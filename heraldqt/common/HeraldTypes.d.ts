declare type ConversationID = number;
declare type UserId = string;

declare class NetworkHandle {
  sendMessage(text: string, conversationID: ConversationID): boolean;
}

declare class Messages {
  conversationId: ConversationID;
  insertMessage(text: string, success: boolean): boolean;
}

declare class Message extends Item {}

declare class Contacts {
  add(userid: UserId): boolean;
}

declare class Config {
  name: UserId;
  configId: string;
  pfpUrl?: string;

  exists(): boolean;
}
