export function isOnscreen(index, chatListView, chatPane, conversationWindow, forward) {
    if (!forward) {
        const item = chatListView.itemAtIndex(index);
        const x = item.x;
        const y = item.y;
        const yPos = chatPane.mapFromItem(chatListView, x, y).y;
        const yPos2 = yPos + item.height;
        const pageHeight = conversationWindow.height;
        return 0 < yPos && yPos2 < pageHeight;
    }
    else {
        const item = chatListView.itemAtIndex(index);
        const x = item.x;
        const y = item.y;
        const yPos = chatPane.mapFromItem(chatListView, x, y).y;
        const yPos2 = yPos + item.height;
        const pageHeight = conversationWindow.height;
        return 0 < yPos && yPos2 < pageHeight;
    }
}
export function searchTextHandler(ownedConversation, chatListView, chatPane, conversationWindow) {
    const index = ownedConversation.prevSearchMatch();
    const onScreen = isOnscreen(index, chatListView, chatPane, conversationWindow, false);
    if (!onScreen) {
        conversationWindow.positionViewAtIndex(index, 1);
        conversationWindow.returnToBounds();
    }
}
export function jumpHandler(ownedConversation, chatListView, chatPane, conversationWindow, forward) {
    const toJump = (index) => {
        return !isOnscreen(index, chatListView, chatPane, conversationWindow, forward);
    };
    if (forward) {
        const index = ownedConversation.nextSearchMatch();
        if (toJump(index)) {
            conversationWindow.positionViewAtIndex(index, 1);
        }
    }
    else {
        const index = ownedConversation.prevSearchMatch();
        if (toJump(index)) {
            conversationWindow.positionViewAtIndex(index, 1);
        }
    }
}
