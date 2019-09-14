declare class ConversationID {}
declare class MessageId {}
declare type UserId = string;

declare class NetworkHandle {
  sendMessage(
    text: string,
    conversationID: ConversationID,
    messageId: MessageId
  ): boolean;
  sendAddRequest(userid: UserId, conversationID: ConversationID): boolean;
}

declare class Messages {
  conversationId: ConversationID;
  insertMessage(text: string, success: boolean): MessageId;
}

declare class Message extends Item {}

declare class Contacts {
  add(userid: UserId): ConversationID;
}

declare const enum ContactStatus {
  Active = 0,
  Archved = 1,
  Deleted = 2
}

declare class Config {
  name: UserId;
  configId: string;
  pfpUrl?: string;
  exists(): boolean;
}
