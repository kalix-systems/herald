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
            chatListView.positionViewAtBeginning();
            break;
        case Qt.Key_End:
            chatListView.positionViewAtEnd();
            break;
        case Qt.Key_Up:
            chatScrollBar.decrease();
            break;
        case Qt.Key_Down:
            chatScrollBar.increase();
            break;
        case Qt.Key_J:
            chatScrollBar.increase();
            break;
        case Qt.Key_K:
            chatScrollBar.decrease();
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
