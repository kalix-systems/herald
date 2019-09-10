declare type ConversationID = number[];

declare class Avatar {}

declare class NetworkHandle {
  sendMessage(text: string, conversationID: ConversationID): boolean;
}

declare class Messages {
  conversationId: ConversationID;
  insertMessage(text: string, success: boolean): boolean;
}

declare class Config {
  name: string;
  configId: string;

  exists(): boolean;
}
