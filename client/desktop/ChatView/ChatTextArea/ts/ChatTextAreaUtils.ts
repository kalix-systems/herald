export function enterKeyHandler(
  event: QKeyEvent,
  target: TextArea,
  builder: MessageBuilder,
  messageModel: Messages,
  textAreaForm: TextAreaForm
): void {
  if (event.modifiers & Qt.ShiftModifier) {
    target.text = target.text + "\n";
    target.cursorPosition = target.text.length;
    return;
  }

  if (
    target.text.trim().length === 0 &&
    !builder.hasDocAttachment &&
    !builder.hasMediaAttachment
  ) {
    return;
  }

  // clear before positional reset
  const text = target.text.trim();
  target.clear();

  builder.body = text;
  builder.finalize();
}

export function appendToTextArea(text: string, target: TextArea): void {
  const position = target.selectionEnd;
  target.insert(position, text);
}
