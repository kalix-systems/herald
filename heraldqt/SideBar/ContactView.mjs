export function contactClickHandler(mouse, contactView, index, convId, optionsMenu, messageModel, appRoot) {
    if (mouse.button === Qt.LeftButton) {
        contactView.currentIndex = index;
        messageModel.conversationId = convId;
        appRoot.gsConversationId = convId;
    }
    else {
        optionsMenu.open();
    }
    return;
}
