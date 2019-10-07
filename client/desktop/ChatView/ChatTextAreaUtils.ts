export function enterKeyHandler(
  event: QKeyEvent,
  target: TextArea,
  networkHandle: NetworkHandle,
  messageModel: Messages
): void {
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
  const messageId = messageModel.sendMessage(text);
}

export function appendToTextArea(text: string, target: TextArea): void {
  const position = target.selectionEnd;
  target.insert(position, text);
}