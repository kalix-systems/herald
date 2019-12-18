export function convWindowKeyHandler(
  event: KeyEvent,
  chatScrollBar: ScrollBar,
  chatListView: ListView<Message>,
  alwaysOnPolicy: ScrollBarPolicy,
  asNeededPolicy: ScrollBarPolicy
): void {
  chatScrollBar.policy = alwaysOnPolicy;
  switch (event.key) {
    case Qt.Key_PageDown:
      chatListView.contentY += chatListView.height;
      break;
    case Qt.Key_PageUp:
      chatListView.contentY -= chatListView.height;
      break;
    case Qt.Key_Home:
      toBeginning(chatScrollBar);
      break;
    case Qt.Key_End:
      toEnd(chatScrollBar);
      break;
    case Qt.Key_Up:
      moveUp(chatScrollBar);
      break;
    case Qt.Key_Down:
      moveDown(chatScrollBar);
      break;
    case Qt.Key_G:
      if (event.modifiers & Qt.ShiftModifier) {
        toEnd(chatScrollBar);
      } else {
        toBeginning(chatScrollBar);
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
      } else {
        chatListView.contentY += chatListView.height;
      }
      break;
  }
  chatScrollBar.policy = asNeededPolicy;
}

function moveDown(chatScrollBar: ScrollBar): void {
  chatScrollBar.increase();
}

function moveUp(chatScrollBar: ScrollBar): void {
  chatScrollBar.decrease();
}

function toEnd(chatScrollBar: ScrollBar): void {
  chatScrollBar.position = 1;
}

function toBeginning(chatScrollBar: ScrollBar): void {
  chatScrollBar.position = 0;
}
