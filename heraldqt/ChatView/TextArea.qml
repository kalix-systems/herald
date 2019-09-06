import QtQuick 2.4
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.2
import LibHerald 1.0
import "ChatTextAreaUtils.js" as CTUtils


TextAreaForm {

    FileDialog {
        id: attachmentsDialogue
        folder: shortcuts.home
        onSelectionAccepted: {
            print("todo: attachments api")
        }
    }

    anchors {
        left: parentPage.left
        right: parentPage.right
        bottom: parentPage.bottom
        margins: QmlCfg.margin
    }

    scrollHeight: Math.min(contentHeight, 100)
}
