declare type ConversationID = number[];

declare class NetworkHandle {
  sendMessage(text: string, conversationID: ConversationID): boolean;
}

declare class Messages {
  conversationId: ConversationID;
  insertMessage(text: string, success: boolean): boolean;
}
