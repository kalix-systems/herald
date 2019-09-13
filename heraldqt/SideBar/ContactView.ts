export function contactItemHeight(visible: boolean): number {
  if (visible) {
    return 60;
  } else {
    return 0;
  }
}

export function contactClickHandler(
  mouse: Qt.MouseEvent,
  contactView: ContactView,
  index: number,
  contactId: number,
  contactItem: ContactItem,
  optionsMenu: Menu,
  messageModel: Messages,
  chatView: ChatView
): void {
  if (mouse.button === Qt.LeftButton) {
    contactView.currentIndex = index;

    console.log(
      contactView.currentIndex,
      contactView.currentItem.contactAvatar.displayName
    );

    contactItem.focus = true;
    messageModel.conversationId = contactId;

  } else {
    optionsMenu.open();
  }
  return;
}
