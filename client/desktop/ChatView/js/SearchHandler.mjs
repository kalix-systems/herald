export function isOnscreen(index, chatListView, chatPane, conversationWindow, forward) {
    if (!forward) {
        const item = chatListView.itemAt(index);
        const x = item.x;
        const y = item.y;
        const yPos = chatPane.mapFromItem(chatListView, x, y).y;
        const pageHeight = conversationWindow.height;
        return 0 < yPos && yPos < pageHeight;
    }
    else {
        const item = chatListView.itemAt(index);
        const x = item.x;
        const y = item.y;
        const yPos = chatPane.mapFromItem(chatListView, x, y).y;
        const pageHeight = conversationWindow.height;
        return 0 < yPos && yPos < pageHeight;
    }
}
export function searchTextHandler(ownedConversation, chatListView, chatPane, conversationWindow) {
    const index = ownedConversation.prevSearchMatch();
    const onScreen = isOnscreen(index, chatListView, chatPane, conversationWindow, false);
    if (!onScreen) {
        const convoMiddle = conversationWindow.height / 2;
        conversationWindow.contentY = chatListView.itemAt(index).y - convoMiddle;
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
            const convoMiddle = conversationWindow.height / 2;
            conversationWindow.contentY = chatListView.itemAt(index).y - convoMiddle;
        }
    }
    else {
        const index = ownedConversation.prevSearchMatch();
        if (toJump(index)) {
            const convoMiddle = conversationWindow.height / 2;
            conversationWindow.contentY = chatListView.itemAt(index).y - convoMiddle;
        }
    }
}
