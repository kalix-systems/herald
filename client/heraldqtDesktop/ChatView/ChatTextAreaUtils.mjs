export function enterKeyHandler(event, target, networkHandle, messageModel, textAreaForm) {
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
    let messageId;
    if (textAreaForm.state === "replystate") {
        messageId = messageModel.reply(text, textAreaForm.replyId);
    }
    else {
        messageId = messageModel.insertMessage(text);
    }
    networkHandle.sendMessage(text, messageModel.conversationId, messageId);
}
export function appendToTextArea(text, target) {
    const position = target.selectionEnd;
    target.insert(position, text);
}
