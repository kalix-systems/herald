declare class ConversationID {}
declare class MessageId {}
declare type UserId = string;

declare class HeraldState {
  configInit: boolean;
  setConfigId(configId: UserId): boolean;
}

declare class NetworkHandle {
  newMessage: boolean;
  connectionUp: boolean;
  connectionPending: boolean;

  sendMessage(
    text: string,
    conversationID: ConversationID,
    messageId: MessageId
  ): boolean;

  sendAddRequest(userid: UserId, conversationID: ConversationID): boolean;

  requestMetaData(of: UserId): boolean;
}

declare class Messages {
  conversationId: ConversationID;
  insertMessage(text: string): MessageId;
  reply(text: string, op: MessageId): MessageId;
  deleteMessage(rowIndex: number): boolean;
  clearConversationView(): void;
  deleteConversation(): boolean;
  deleteConversationById(conversationId: ConversationID): boolean;
}

declare class Message extends Item {}

declare class Contacts {
  add(userid: UserId): ConversationID;
  indexFromConversationId(conversationID: ConversationID): number;
  toggleFilterRegexFilterRegex(): boolean;
}

declare const enum ContactStatus {
  Active = 0,
  Archved = 1,
  Deleted = 2
}

declare class Config {
  configId: UserId;
  name: string;
  pfpUrl?: string;
  // TODO replace this number with a const enum
  color: number;
  colorscheme: ColorScheme;
}
