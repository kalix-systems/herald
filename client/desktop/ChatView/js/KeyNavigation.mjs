export function convWindowKeyHandler(event, chatScrollBar, chatListView, alwaysOnPolicy, asNeededPolicy) {
    chatScrollBar.policy = alwaysOnPolicy;
    switch (event.key) {
        case Qt.Key_PageDown:
            chatListView.contentY += chatListView.height;
            break;
        case Qt.Key_PageUp:
            chatListView.contentY -= chatListView.height;
            break;
        case Qt.Key_Home:
            toBeginning(chatListView);
            break;
        case Qt.Key_End:
            toEnd(chatListView);
            break;
        case Qt.Key_Up:
            moveUp(chatScrollBar);
            break;
        case Qt.Key_Down:
            moveDown(chatScrollBar);
            break;
        case Qt.Key_G:
            if (event.modifiers & Qt.ShiftModifier) {
                toEnd(chatListView);
            }
            else {
                toBeginning(chatListView);
            }
            break;
        case Qt.Key_J:
            moveDown(chatScrollBar);
            break;
        case Qt.Key_K:
            moveUp(chatScrollBar);
            break;
        case Qt.Key_Space:
            if (event.modifiers & Qt.ShiftModifier) {
                chatListView.contentY -= chatListView.height;
            }
            else {
                chatListView.contentY += chatListView.height;
            }
            break;
    }
    chatScrollBar.policy = asNeededPolicy;
}
function moveDown(chatScrollBar) {
    chatScrollBar.increase();
}
function moveUp(chatScrollBar) {
    chatScrollBar.decrease();
}
function toEnd(chatListView) {
    chatListView.positionViewAtBeginning();
}
function toBeginning(chatListView) {
    chatListView.positionViewAtEnd();
}
