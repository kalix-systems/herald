import QtQuick 2.4
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.2
import LibHerald 1.0
import "ChatTextAreaUtils.js" as CTUtils


TextAreaForm {
        id: self

   FileDialog {
        id: attachmentsDialogue
        folder: shortcuts.home
        onSelectionAccepted: {
            print("todo: attachments api")
        }
    }

    keysProxy: Item {
        Keys.onReturnPressed: CTUtils.enterHandler(event, self.chatText)
    }

    atcButton.onClicked: attachmentsDialogue.open()
    scrollHeight: Math.min(contentHeight, 100)
}
