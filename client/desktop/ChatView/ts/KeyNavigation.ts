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
      } else {
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

function toEnd(chatListView: ListView<Message>): void {
  // this is a workaround to the ListView's efficiency heurisitics
  // if someone else knows a better way, please fix this. This doesn't even
  // quite work
  chatListView.positionViewAtEnd();
  chatListView.forceLayout();
  chatListView.positionViewAtEnd();
}

function toBeginning(chatListView: ListView<Message>): void {
  // this is a workaround to the ListView's efficiency heurisitics
  // if someone else knows a better way, please fix this
  chatListView.contentY = 0;
  chatListView.forceLayout();
  chatListView.contentY = 0;
}
