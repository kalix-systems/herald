export function isOnscreen(
  index: number,
  chatListView: ListView<Item>,
  chatPane: Page,
  conversationWindow: ConversationWindow,
  forward: boolean
): boolean {
  if (!forward) {
    const item = chatListView.itemAtIndex(index);
    const x = item.x;
    const y = item.y;

    const yPos = chatPane.mapFromItem(chatListView, x, y).y;
    const yPos2 = yPos + item.height;
    const pageHeight = conversationWindow.height;

    return 0 < yPos && yPos2 < pageHeight;
  } else {
    const item = chatListView.itemAtIndex(index);
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
  chatListView: ListView<Item>,
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
    conversationWindow.positionViewAtIndex(index, ListView.Center)
    conversationWindow.returnToBounds();
  }
}

export function jumpHandler(
  ownedConversation: Messages,
  chatListView: ListView<Item>,
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
      conversationWindow.positionViewAtIndex(index, ListView.Center)

    }
  } else {
    const index = ownedConversation.prevSearchMatch();

    if (toJump(index)) {
      conversationWindow.positionViewAtIndex(index, ListView.Center)
    }
  }
}
