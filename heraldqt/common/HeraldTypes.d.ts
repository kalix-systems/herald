declare type ConversationID = number[];
declare type UserId = string;

declare class Avatar {}

declare class NetworkHandle {
  sendMessage(text: string, conversationID: ConversationID): boolean;
}

declare class Messages {
  conversationId: ConversationID;
  insertMessage(text: string, success: boolean): boolean;
}

declare class Contacts {
  add(userid: UserId): boolean;
}

declare class Config {
  name: UserId;
  configId: string;

  exists(): boolean;
}
