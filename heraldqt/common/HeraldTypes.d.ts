declare class ByteArray {}

declare class ConversationID extends ByteArray {}
declare class MessageId extends ByteArray {}

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
  setStatus(rowIndex: number, status: ContactStatus): boolean;
  setName(rowIndex: number, name: string): boolean;
  pairwiseConversationId(rowIndex: number): ConversationID;
  indexFromConversationId(conversationID: ConversationID): number;
  toggleFilterRegexFilterRegex(): boolean;
}

declare class Contact {
  contactId: UserId;
  pairwiseConversationId: ConversationID;
  name?: string;
  profilePicture?: string;
  // TODO const enum for colors
  color: number;
  status: ContactStatus;
  matched: boolean;
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

declare class HeraldUtils {
  compareByteArray(bs1: ByteArray, bs2: ByteArray): boolean;
}
