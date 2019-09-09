import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "ChatView" as CVUtils
import "common/utils.js" as Utils
import "ChatView/ChatTextAreaUtils.js" as CTUtils


Pane {
    id: chatPane
    enabled: false
    opacity: 0
    padding: 0
    property alias messageBar: messageBar

    /// bar at the top that displays the avatar
    CVUtils.ChatBar {
        id: messageBar
    }

    ///--- border between messageBar and main chat view
    Rectangle {
        height: 1
        color: QmlCfg.palette.secondaryColor
        anchors {
            top: messageBar.bottom
            left: parent.left
            right: parent.right
        }
    }

    ///--- chat view, shows messages
   CVUtils.ConversationWindowForm {
       id: convWindow
       focus: true
       anchors {
           top: messageBar.bottom
           bottom: chatTextArea.top
           left: parent.left
           right: parent.right
       }
       Component.onCompleted: forceActiveFocus()
       Keys.onUpPressed: chatScrollBar.decrease()
       Keys.onDownPressed: chatScrollBar.increase()
       Connections {
           target: messageModel
           onRowsInserted: {
               convWindow.contentY = convWindow.contentHeight
           }
       }
    }

    ///--- Text entry area, for typing
    CVUtils.TextAreaForm {
        id: chatTextArea
        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
            margins: QmlCfg.margin
        }

        keysProxy: Item {
                Keys.onReturnPressed: CTUtils.enterHandler(event, chatTextArea.chatText)
                // TODO: Tab should cycle through a hierarchy of items as far as focus
            }
        emojiButton.onClicked: print("placeholder until emoji pop up")
        atcButton.onClicked: attachmentsDialogue.open()
        scrollHeight: Math.min(contentHeight, 100)
    }

    states: State {
        name: "visibleview"
        PropertyChanges {
            target: chatPane
            opacity: 100
            enabled: true
        }
    }
}
