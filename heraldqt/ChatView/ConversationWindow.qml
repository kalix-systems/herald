import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "." as CVUtils
import "../common/utils.js" as Utils

CVUtils.ConversationWindowForm {

    Connections {
        target: messageModel
        onRowsInserted: {
            contentY = contentHeight
        }
    }

    Component.onCompleted: forceActiveFocus()
    Keys.onUpPressed: chatScrollBar.decrease()
    Keys.onDownPressed: chatScrollBar.increase()
}
