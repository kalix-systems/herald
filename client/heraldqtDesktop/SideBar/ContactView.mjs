export function contactClickHandler(mouse, contactView, index, optionsMenu) {
    if (mouse.button === Qt.LeftButton) {
        contactView.currentIndex = index;
    }
    else {
        optionsMenu.open();
    }
    return;
}
