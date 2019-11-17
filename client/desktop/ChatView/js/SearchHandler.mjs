export function isOnscreen(ownedConversation, chatListView, chatPane, conversationWindow, forward) {
    if (!forward) {
        const x = chatListView.itemAt(ownedConversation.peekPrevSearchMatch()).x;
        const y = chatListView.itemAt(ownedConversation.peekPrevSearchMatch()).y;
        const yPos = chatPane.mapFromItem(chatListView, x, y).y;
        const pageHeight = conversationWindow.height;
        if (0 < yPos && yPos < pageHeight) {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        const x = chatListView.itemAt(ownedConversation.peekNextSearchMatch()).x;
        const y = chatListView.itemAt(ownedConversation.peekNextSearchMatch()).y;
        const yPos = chatPane.mapFromItem(chatListView, x, y).y;
        const pageHeight = conversationWindow.height;
        if (0 < yPos && yPos < pageHeight) {
            return true;
        }
        else {
            return false;
        }
    }
}
export function jumpHandler(ownedConversation, chatListView, chatPane, conversationWindow, forward) {
    const toJump = !isOnscreen(ownedConversation, chatListView, chatPane, conversationWindow, forward);
    const convoMiddle = conversationWindow.height / 2;
    if (forward) {
        if (toJump) {
            conversationWindow.contentY = chatListView.itemAt(ownedConversation.nextSearchMatch()).y - convoMiddle;
        }
        else {
            ownedConversation.nextSearchMatch();
        }
        return;
    }
    else {
        if (toJump) {
            conversationWindow.contentY = chatListView.itemAt(ownedConversation.prevSearchMatch()).y - convoMiddle;
        }
        else {
            ownedConversation.prevSearchMatch();
        }
        return;
    }
}
