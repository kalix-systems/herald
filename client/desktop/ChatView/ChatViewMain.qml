import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "./ConversationWindow" as ConvoWindow
import "./ChatTextArea" as CTA
import "./Header" as Header
import "./ChatTextArea/js/ChatTextAreaUtils.mjs" as TextJs
import "js/KeyNavigation.mjs" as KeyNav
import "qrc:/imports/EmojiKeyboard" as EK
import "../common" as Common
import "Popups" as Popups
import QtQuick.Dialogs 1.2
import "qrc:/imports/ChatBubble" as CB

//import Qt.labs.platform 1.1
Page {
    id: chatPage

    //TODO: rename this to something sane
    property var conversationItem
    //TODO: rename to something sane and not a shadow
    property Messages ownedConversation
    property Members conversationMembers
    property alias convoTimer: messageBar.timerMenu
    property var convId

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    header: Header.ChatBar {
        id: messageBar
        conversationItem: parent.conversationItem
    }

    Header.ChatSearchComponent {
        id: chatSearchComponent
    }

    Common.Divider {
        anchors.top: parent.top
        color: CmnCfg.palette.borderColor
        z: messageBar.z
    }
    //TODO: Rename to MessagesView
    ConvoWindow.ConversationWindow {
        id: convWindow
        focus: true
        anchors {
            top: parent.top
            bottom: divider.top
            left: parent.left
            right: parent.right
        }

        Component.onCompleted: forceActiveFocus()
        Keys.onPressed: KeyNav.convWindowKeyHandler(event, chatScrollBar,
                                                    convWindow,
                                                    ScrollBar.AlwaysOn,
                                                    ScrollBar.AsNeeded)

        Connections {
            target: conversationList
            onMessagePositionRequested: {
                const msg_idx = chatPage.ownedConversation.indexById(
                                  requestedMsgId)
                // early return on out of bounds
                if ((msg_idx < 0) || (msg_idx >= convWindow.count))
                    return

                convWindow.positionViewAtIndex(msg_idx, ListView.Center)
                convWindow.highlightAnimation.target = convWindow.itemAtIndex(
                            msg_idx).highlightItem
                convWindow.highlightAnimation.start()
            }
        }
    }

    // wrapper Item to set margins for the popup instead of
    // having to use explicit x and y positioning
    Item {
        anchors {
            right: parent.right
            bottom: chatTextArea.top
            margins: 12
        }
        height: emoKeysPopup.height
        width: emoKeysPopup.width
        Popup {
            id: emojiPopupWrapper
            onOpened: emoKeysPopup.active = true
            onClosed: emoKeysPopup.active = false
            height: emoKeysPopup.height
            width: emoKeysPopup.width

            Popups.EmojiPopup {
                id: emoKeysPopup
                anchors.centerIn: parent
                onActiveChanged: if (!active)
                                     emojiPopupWrapper.close()
            }
        }
    }

    Common.Divider {
        id: divider
        height: 1
        color: CmnCfg.palette.black
        anchors.bottom: chatTextArea.top
        Drawer {
            id: drawer
            width: parent.width
            height: CmnCfg.typeMargin
        }
    }

    ///--- Text entry area, for typing
    CTA.ConvoTextArea {
        id: chatTextArea

        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
            topMargin: CmnCfg.defaultMargin
            bottomMargin: 0
            leftMargin: 0
            rightMargin: 0
        }
        //to handle jumping behavior in bubbles caused when the page is small
        onHeightChanged: {
            if (convWindow.height > convWindow.contentHeight) {
                convWindow.height = convWindow.contentHeight
            }
        }
        keysProxy: Item {
            Keys.onReturnPressed: {
                TextJs.enterKeyHandler(event, chatTextArea.chatText,
                                       ownedConversation.builder,
                                       ownedConversation, chatTextArea)
                // TODO: Tab should cycle through a hierarchy of items as far as focus
            }
        }
        emojiButton.onClicked: emojiPopupWrapper.open()
        atcButton.onClicked: chatTextArea.attachmentsDialogue.open()
    }

    CB.TypingBubble {

        id: typingIndicator
        anchors.bottom: divider.top
        conversationMembers: ContentMap.get(chatPage.convId).members
    }

    MessageDialog {
        id: clearHistoryPrompt
        text: qsTr("Clear conversation history")
        informativeText: qsTr("Do you want to clear this conversation's history from this device?")
        standardButtons: MessageDialog.Ok | MessageDialog.Cancel

        onAccepted: {
            ownedConversation.clearConversationHistory()
        }
    }

    MessageDialog {
        id: deleteMsgPrompt
        property var deleteId
        text: qsTr("Delete message")
        informativeText: qsTr("Do you want to delete this message from this device?")
        standardButtons: MessageDialog.Ok | MessageDialog.Cancel

        onAccepted: {
            // prevent coercion of undefined into bytearray
            if (deleteId === undefined) {
                return
            }
            ownedConversation.deleteMessageById(deleteId)
            deleteId = undefined
        }
    }
}
