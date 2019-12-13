export function contactClickHandler(
  mouse: Qt.MouseEvent,
  contactView: ContactView,
  index: number,
  optionsMenu: Menu
): void {
  if (mouse.button === Qt.LeftButton) {
    contactView.currentIndex = index;
  } else {
    optionsMenu.open();
  }
  return;
}

export function formatSummary(author: string, body: string): string {
  if (author) {
    return author + ": " + body;
  } else {
    return "";
  }
}
