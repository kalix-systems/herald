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
