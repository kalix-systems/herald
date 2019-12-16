export function contactClickHandler(mouse, contactView, index, optionsMenu) {
    if (mouse.button === Qt.LeftButton) {
        contactView.currentIndex = index;
    }
    else {
        optionsMenu.open();
    }
    return;
}
export function formatSummary(author, body) {
    if (author) {
        return author + ": " + body;
    }
    else {
        return "";
    }
}
