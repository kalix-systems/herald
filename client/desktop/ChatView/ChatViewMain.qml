import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "." as CVUtils
import "../../foundation/js/utils.mjs" as Utils
import "./Controls/js/ChatTextAreaUtils.mjs" as JS
import "./Controls/ConvoTextArea"
import "../EmojiKeyboard" as EK
import "../common" as Common
import "Popups" as Popups

Page {
    id: chatPane

    property var conversationItem
    property Messages ownedConversation

    background: Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.mainColor
    }

    header: CVUtils.ChatBar {
        id: messageBar
        conversationItem: parent.conversationItem
    }

    Common.Divider {
        anchors.top: parent.top
        color: CmnCfg.palette.borderColor
        z: messageBar.z
    }

    CVUtils.ConversationWindow {
        id: convWindow
        focus: true
        anchors {
            top: parent.top
            bottom: chatTextArea.top
            left: parent.left
            right: parent.right
        }
        Component.onCompleted: forceActiveFocus()
        Keys.onUpPressed: chatScrollBar.decrease()
        Keys.onDownPressed: chatScrollBar.increase()
        Connections {
            target: ownedConversation
            onRowsInserted: {
                convWindow.contentY = convWindow.contentHeight
            }
        }
    }

    // This should be spawned by the EK
    MouseArea {
        id: exit
        enabled: emoKeysPopup.active
        anchors.fill: parent
        onClicked: {
            emoKeysPopup.active = false
            anchors.fill = undefined
        }
    }

    Popups.EmojiPopup {
        id: emoKeysPopup
        anchors.bottom: chatTextArea.top
        anchors.left: chatTextArea.left
    }

    Common.Divider {
        height: 1
        color: CmnCfg.palette.borderColor
        bottomAnchor: chatTextArea.top
    }

    ///--- Text entry area, for typing
    ConvoTextArea {
        id: chatTextArea

        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
            margins: CmnCfg.margin
            bottomMargin: CmnCfg.smallMargin
        }

        keysProxy: Item {
            MessageBuilder {
                id: builder
            }
            Keys.onReturnPressed: JS.enterKeyHandler(
                                      event, chatTextArea.chatText, builder,
                                      // this is actually a text area TODO rename
                                      ownedConversation, chatTextArea)
            // TODO: Tab should cycle through a hierarchy of items as far as focus
        }
        emojiButton.onClicked: emoKeysPopup.active = !!!emoKeysPopup.active
        atcButton.onClicked: chatTextArea.attachmentsDialogue.open()
    }
}
