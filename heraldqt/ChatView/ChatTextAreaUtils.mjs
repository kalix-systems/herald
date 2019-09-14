/*
 * a memory tuple class
 **/
export class ChatViewMemory {
    constructor() {
        this.scrollMemory = Number(1.0);
        this.textMemory = String("");
    }
}
/*
 * global object to keep track of  chat area memory
 **/
export const textAreaMemory = {
    currentConversationid: "",
    invalid: new ChatViewMemory()
};
/*
 * Gets chatview memory with the current conversationID
 * on falsey conversation id returns
 **/
export function getTextAreaMemory(conversationID) {
    if (!!!conversationID) {
        return textAreaMemory.invalid;
    }
}
export function enterKeyHandler(event, target, networkHandle, messageModel) {
    if (event.modifiers & Qt.ShiftModifier) {
        target.text = target.text + "\n";
        target.cursorPosition = target.text.length;
        return;
    }
    if (target.text.trim().length <= 0) {
        return;
    }
    // clear before positional reset
    const text = target.text;
    target.clear();
    const message_id = messageModel.insertMessage(text, false);
    networkHandle.sendMessage(text, messageModel.conversationId, message_id);
}
