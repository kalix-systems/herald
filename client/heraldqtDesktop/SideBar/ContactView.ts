export function contactClickHandler(
  mouse: Qt.MouseEvent,
  contactView: ContactView,
  index: number,
  convId: ConversationID, // TODO don't use this
  optionsMenu: Menu,
  messageModel: Messages,
  appRoot: GlobalState
): void {
  if (mouse.button === Qt.LeftButton) {
    contactView.currentIndex = index;
    // messageModel.conversationId = convId;
  } else {
    optionsMenu.open();
  }
  return;
}
