export function contactItemHeight(visible) {
    if (visible) {
        return 60;
    }
    else {
        return 0;
    }
}
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
