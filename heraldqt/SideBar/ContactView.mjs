export function contactItemHeight(visible) {
    if (visible) {
        return 60;
    }
    else {
        return 0;
    }
}
export function contactClickHandler(mouse, contactView, index, contactId, contactItem, optionsMenu, messageModel, chatView) {
    if (mouse.button === Qt.LeftButton) {
        contactView.currentIndex = index;
        console.log(contactView.currentIndex, contactView.currentItem.contactAvatar.displayName);
        contactItem.focus = true;
        messageModel.conversationId = contactId;
    }
    else {
        optionsMenu.open();
    }
    return;
}
