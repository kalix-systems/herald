export declare class ChatViewMemory {
    scrollMemory: number;
    textMemory: string;
    constructor();
}
export declare const textAreaMemory: {
    currentConversationid: string;
    invalid: ChatViewMemory;
};
export declare function getTextAreaMemory(conversationID: number): ChatViewMemory | undefined;
export declare function enterKeyHandler(event: QKeyEvent, target: TextArea, networkHandle: NetworkHandle, messageModel: Messages): void;
