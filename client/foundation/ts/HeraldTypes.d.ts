declare class ByteArray {}

declare class ConversationID extends ByteArray {}
declare class MessageId extends ByteArray {}

declare const enum ExpirationPeriod {
  Never = 0,
  OneMinute = 1,
  OneHour = 2,
  OneDay = 3,
  OneWeek = 4,
  OneMonth = 5,
  OneYear = 6
}

declare type UserId = string;

declare class HeraldState {
  configInit: boolean;
}

declare class NetworkHandle {
  connectionUp: boolean;
  connectionPending: boolean;

  registerNewUser(userid: UserId): boolean;
  login(): boolean;
}

declare class Messages {
  conversationId?: ConversationID;
  lastAuthor: string;
  lastBody: string;
  lastEpochTimestampMs: number;
  deleteMessage(rowIndex: number): boolean;
  clearConversationHistory(): void;
  deleteConversation(): boolean;
  deleteConversationById(conversationId: ConversationID): boolean;
}

declare class MessageBuilder {
  conversationId?: ConversationID;
  replyingTo?: MessageId;
  body?: string;
  finalize(): void;
  addAttachment(path: string): boolean;
  removeAttachment(path: string): boolean;
  removeAttachmentByIndex(rowIndex: number): boolean;
  removeLast(): void;
  attachmentPath(rowIndex: number): string;
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
}

declare class ConversationBuilder {
  addMember(userId: UserId): boolean;
  removeMemberById(userId: UserId): boolean;
  removeMemberByIndex(rowIndex: number): boolean;
  removeLast(): void;
  finalize(): ByteArray;
}

declare class HeraldUtils {
  compareByteArray(
    bs1: ByteArray | undefined,
    bs2: ByteArray | undefined
  ): boolean;
  isValidRandId(bs: ByteArray): boolean;
}
