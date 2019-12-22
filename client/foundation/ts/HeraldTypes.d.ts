declare class ByteArray {}

declare class ConversationID extends ByteArray {}
declare class MsgId extends ByteArray {}
declare class MessageSearch {}

declare const enum ExpirationPeriod {
  // Messages never expire
  Never = 0,
  // Messages expire after 30 seconds
  ThirtySeconds = 1,
  // Messages expire after one minute
  OneMinute = 2,
  // Messages expire after one minute
  ThirtyMinutes = 3,
  // Messages expire after one hour
  OneHour = 4,
  // Messages expire after twelve hours
  TwelveHours = 5,
  // Messages expire after one day
  OneDay = 6,
  // Message expire after one week
  OneWeek = 7,
  // Messages expire after one month
  OneMonth = 8,
  // Messages expire after one year
  OneYear = 9
}

declare const enum MessageReceiptStatus {
  /// Not acknowledged
  NoAck = 0,
  /// Received by user
  Received = 1,
  /// Read by the recipient
  Read = 2,
  /// The user has read receipts turned off
  AckTerminal = 3
}

declare const enum RegistrationFailureCode {
  UserIdTaken = 0,
  KeyTaken = 1,
  BadSignature = 2,
  Other = 3
}

declare const enum MatchStatus {
  NotMatched = 0,
  Matched = 1,
  Focused = 2
}

declare const enum ReplyType {
  None = 0,
  Dangling = 1,
  Known = 2
}

declare type UserId = string;

declare class Herald {
  configInit: boolean;

  connectionUp: boolean;
  connectionPending: boolean;

  config: Config;
  conversationBuilder: ConversationBuilder;
  conversations: Conversations;
  messageSearch: MessageSearch;
  users: Users;
  usersSearch: Users;
  utils: Utils;

  registerNewUser(userid: UserId): boolean;
  login(): boolean;
}

declare class Messages {
  conversationId?: ConversationID;
  lastAuthor: string;
  lastBody: string;
  isEmpty: string;
  lastTime: number;
  builder: MessageBuilder;

  deleteMessage(rowIndex: number): boolean;
  clearConversationHistory(): void;
  deleteConversation(): boolean;
  deleteConversationById(conversationId: ConversationID): boolean;

  searchPattern: string;
  searchActive: boolean;
  searchNumMatches: number;
  searchIndex?: number;
  prevSearchMatch(): number;
  nextSearchMatch(): number;

  saveAllAttachments(index: number, dest: string): boolean;
}

declare class MessageBuilder {
  isReply: boolean;
  body?: string;
  isMediaMessage: boolean;
  parseMarkdown: boolean;

  opId?: MsgId;
  opAuthor?: UserId;
  opBody?: string;
  opTime?: number;
  opHasAttachments?: boolean;

  finalize(): void;
  addAttachment(path: string): boolean;
  removeAttachment(path: string): boolean;
  removeAttachmentByIndex(index: number): boolean;
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

declare class Utils {
  compareByteArray(
    bs1: ByteArray | undefined,
    bs2: ByteArray | undefined
  ): boolean;
  isValidRandId(bs: ByteArray): boolean;
}
