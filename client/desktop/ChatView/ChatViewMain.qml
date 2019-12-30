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

Page {
    id: chatPage

    //TODO: rename this to something sane
    property var conversationItem
    //TODO: rename to something sane and not a shadow
    property var ownedConversation
    property var conversationMembers

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
            bottom: chatTextArea.top
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
        anchors {
            margins: 12
            bottom: chatTextArea.top
            left: parent.left
        }
    }

    Common.Divider {
        height: 1
        color: CmnCfg.palette.black
        anchors.bottom: chatTextArea.top
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
        onHeightChanged: convWindow.height = convWindow.contentHeight
        keysProxy: Item {
            Keys.onReturnPressed: TextJs.enterKeyHandler(
                                      event, chatTextArea.chatText,
                                      ownedConversation.builder,
                                      ownedConversation, chatTextArea)
            // TODO: Tab should cycle through a hierarchy of items as far as focus
        }
        emojiButton.onClicked: emoKeysPopup.active = !!!emoKeysPopup.active
        atcButton.onClicked: chatTextArea.attachmentsDialogue.open()
    }
}
