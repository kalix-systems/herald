import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "." as CVUtils
import "../common/utils.mjs" as Utils
import "./ChatTextAreaUtils.mjs" as CTUtils
import "../common" as Common

Pane {
    id: chatPane
    padding: 0
    property alias messageBar: messageBar
    property Conversation ownedConversation
    property var conversationId : null


    /// bar at the top that displays the avatar
    CVUtils.ChatBar {
        id: messageBar
    }

    Common.Divider {
        height: 1
        anchors.bottom: messageBar.bottom
        color: "black"
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
        //Connections {
        //    target:convModel
        //    onRowsInserted: {
        //        convWindow.contentY = convWindow.contentHeight
        //    }
        //}
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
            Keys.onReturnPressed: CTUtils.enterKeyHandler(
                                      event, chatTextArea.chatText,
                                      networkHandle, ownedConversation)
            // TODO: Tab should cycle through a hierarchy of items as far as focus
        }
        emojiButton.onClicked: print("placeholder until emoji pop up")
        atcButton.onClicked: chatTextArea.attachmentsDialogue.open()
        scrollHeight: Math.min(contentHeight, 100)
    }
}
