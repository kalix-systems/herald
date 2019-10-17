export function enterKeyHandler(event, target, builder, messageModel, textAreaForm) {
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
    builder.body = text;
    builder.conversationId = messageModel.conversationId;
    builder.finalize();
    textAreaForm.state = "default";
}
export function appendToTextArea(text, target) {
    const position = target.selectionEnd;
    target.insert(position, text);
}
