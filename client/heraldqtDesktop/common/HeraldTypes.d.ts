declare class ByteArray {}

declare class ConversationID extends ByteArray {}
declare class MessageId extends ByteArray {}

declare type UserId = string;

declare class HeraldState {
  configInit: boolean;
}

declare class NetworkHandle {
  connectionUp: boolean;
  connectionPending: boolean;

  msgData: number;
  usersData: number;
  membersData: number;
  convData: number;

  sendAddRequest(userid: UserId, conversationID: ConversationID): boolean;
  registerNewUser(userid: UserId): boolean;
  login(): boolean;
}

declare class Messages {
  conversationId: ConversationID;
  sendMessage(text: string): MessageId;
  reply(text: string, op: MessageId): MessageId;
  deleteMessage(rowIndex: number): boolean;
  clearConversationView(): void;
  deleteConversation(): boolean;
  deleteConversationById(conversationId: ConversationID): boolean;
}

declare class Message extends Item {}

declare class Users {
  add(userid: UserId): ConversationID;
  setStatus(rowIndex: number, status: ContactStatus): boolean;
  setName(rowIndex: number, name: string): boolean;
  setProfilePicture(rowIndex: number, profilePicture: string): boolean;
  pairwiseConversationId(rowIndex: number): ConversationID;
  indexFromConversationId(conversationID: ConversationID): number;
  toggleFilterRegexFilterRegex(): boolean;
}

declare class User {
  userId: UserId;
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

declare class Conversations {
  filter: string;
  filterRegex: string;
  toggleFilterRegex(): boolean;
  addConversation(): ByteArray;
  removeConversation(rowIndex: number): boolean;
  pollUpdate(): boolean;
}

declare class HeraldUtils {
  compareByteArray(bs1: ByteArray, bs2: ByteArray): boolean;
  chatBubbleNaturalWidth(chat_pane_width: number, text_width: number): number;
  isValidRandId(bs: ByteArray): boolean;
}
