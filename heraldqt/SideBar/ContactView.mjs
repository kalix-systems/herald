export function contactClickHandler(mouse, contactView, index, convId, optionsMenu, messageModel) {
    if (mouse.button === Qt.LeftButton) {
        contactView.currentIndex = index;
        messageModel.conversationId = convId;
    }
    else {
        optionsMenu.open();
    }
    return;
}
