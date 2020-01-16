import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "./ConversationWindow" as ConvoWindow
import "./ChatTextArea" as CTA
import "./Header" as Header
import "./ChatTextArea/js/ChatTextAreaUtils.mjs" as TextJs
import "js/KeyNavigation.mjs" as KeyNav
import "../EmojiKeyboard" as EK
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
    property var ownedConversation
    property var conversationMembers
    property alias convoTimer: messageBar.timerMenu

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
            left: parent.left
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

    //item that wraps type bubble; will eventually wrap a flow in the loader to show multiple typing indicators
    Rectangle {
        id: typingIndicator
        anchors.bottom: divider.top
        height: typingLoader.height
        width: parent.width
        color: typingLoader.active ? CmnCfg.palette.white : "transparent"
        Rectangle {
            visible: typingLoader.active
            anchors.top: parent.top
            width: parent.width
            height: 1
            color: CmnCfg.palette.medGrey
        }

        Connections {
            target: conversationMembers
            onNewTypingIndicator: {
                typingIndicator.__secondsSinceLastReset = 0
            }
        }

        Connections {
            target: appRoot.globalTimer
            onRefreshTime: {
                if (typingLoader.active)
                    typingIndicator.__secondsSinceLastReset += 1
            }
        }

        property int __secondsSinceLastReset: 5
        property bool __aUserIsTyping: __secondsSinceLastReset < 4

        property string typeText
        Connections {
            target: conversationMembers
            onNewTypingIndicator: {
                typingIndicator.typeText = ""
                if (conversationMembers.typingMembers() === "") {
                    typingLoader.active = false
                    return
                }
                const typers = JSON.parse(conversationMembers.typingMembers())

                const num = typers.length
                const last = num - 1

                if (num <= 0) {
                    typingLoader.active = false
                    return
                }

                typers.forEach(function (item, index) {
                    const typingUserName = item
                    if (num === 1) {
                        typingIndicator.typeText += typingUserName + qsTr(
                                    " is typing...")
                        return
                    }

                    if (num > 4) {
                        typingIndicator.typeText = "Several people are typing..."
                        return
                    }
                    if (num === 2 && index === 0) {
                        typingIndicator.typeText += typingUserName + qsTr(
                                    " and ")
                        return
                    }

                    if (index < last - 1) {
                        typingIndicator.typeText += typingUserName + ", "
                        return
                    }

                    if (index < last) {
                        typingIndicator.typeText += typingUserName + " and "
                        return
                    }

                    typingIndicator.typeText += typingUserName + qsTr(
                                " are typing...")
                    return
                })
            }
        }

        Loader {
            id: typingLoader
            active: typingIndicator.__aUserIsTyping
            asynchronous: true

            height: CmnCfg.typeMargin

            width: parent.width
            anchors.bottom: parent.bottom
            sourceComponent: Label {
                id: typeLabel

                text: typingIndicator.typeText
            }
        }
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
