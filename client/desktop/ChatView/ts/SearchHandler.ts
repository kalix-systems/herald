export function isOnscreen(
  index: number,
  chatListView: Repeater,
  chatPane: Page,
  conversationWindow: ConversationWindow,
  forward: boolean
): boolean {
  if (!forward) {
    const item = chatListView.itemAt(index);
    const x = item.x;
    const y = item.y;

    const yPos = chatPane.mapFromItem(chatListView, x, y).y;
    const yPos2 = yPos + item.height;
    const pageHeight = conversationWindow.height;

    return 0 < yPos && yPos2 < pageHeight;
  } else {
    const item = chatListView.itemAt(index);
    const x = item.x;
    const y = item.y;

    const yPos = chatPane.mapFromItem(chatListView, x, y).y;
    const yPos2 = yPos + item.height;
    const pageHeight = conversationWindow.height;

    return 0 < yPos && yPos2 < pageHeight;
  }
}

export function searchTextHandler(
  ownedConversation: Messages,
  chatListView: Repeater,
  chatPane: Page,
  conversationWindow: ConversationWindow
): void {
  const index = ownedConversation.prevSearchMatch();
  const onScreen = isOnscreen(
    index,
    chatListView,
    chatPane,
    conversationWindow,
    false
  );

  if (!onScreen) {
    const convoMiddle = conversationWindow.height / 2;
    conversationWindow.contentY = chatListView.itemAt(index).y - convoMiddle;

    conversationWindow.returnToBounds();
  }
}

export function jumpHandler(
  ownedConversation: Messages,
  chatListView: Repeater,
  chatPane: Page,
  conversationWindow: ConversationWindow,
  forward: boolean
): void {
  const toJump = (index: number): boolean => {
    return !isOnscreen(
      index,
      chatListView,
      chatPane,
      conversationWindow,
      forward
    );
  };

  if (forward) {
    const index = ownedConversation.nextSearchMatch();

    if (toJump(index)) {
      const convoMiddle = conversationWindow.height / 2;
      conversationWindow.contentY = chatListView.itemAt(index).y - convoMiddle;
    }
  } else {
    const index = ownedConversation.prevSearchMatch();

    if (toJump(index)) {
      const convoMiddle = conversationWindow.height / 2;
      conversationWindow.contentY = chatListView.itemAt(index).y - convoMiddle;
    }
  }
}
