export function contactClickHandler(
  mouse: Qt.MouseEvent,
  contactView: ContactView,
  index: number,
  convId: ConversationID,
  optionsMenu: Menu,
  messageModel: Messages,
  appRoot: GlobalState
): void {
  if (mouse.button === Qt.LeftButton) {
    contactView.currentIndex = index;
    messageModel.conversationId = convId;
    appRoot.gsConversationId = convId;
  } else {
    optionsMenu.open();
  }
  return;
}
