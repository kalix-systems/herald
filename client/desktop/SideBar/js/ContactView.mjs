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
//see herald_common/types.rs
export function receiptStatusSwitch(receipt) {
    switch (receipt) {
        case 0: {
            // animated svg in the future
            return "";
        }
        case 1: {
            return "qrc:/single-check-receipt-icon.svg";
        }
        case 2: {
            return "qrc:/double-check-receipt-icon.svg";
        }
        case 3: {
            return "qrc:/single-check-receipt-icon.svg";
        }
    }
    return "";
}
