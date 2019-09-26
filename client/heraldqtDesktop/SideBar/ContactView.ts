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
    return author + ": " + body
  } else {
    return ""
  }
}

//see herald_common/types.rs
export function receiptStatusSwitch(receipt: number): string {
  switch (receipt) {
    case 0: {
      // animated svg in the future
      return ""
    }
    case 1: {
      return "qrc:/single-check-receipt-icon.svg"
    }
    case 2: {
      return "qrc:/double-check-receipt-icon.svg"
    }
    case 3: {
      return "qrc:/single-check-receipt-icon.svg"
    }
  }
  return ""
}
