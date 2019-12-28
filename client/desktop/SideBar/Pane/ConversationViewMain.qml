import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "qrc:/common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "../../ChatView" as CV
import ".././js/ContactView.mjs" as JS
import "../popups" as Popups
import Qt.labs.platform 1.1

/// --- displays a list of conversations
/// TODO: fix bounds bounds behavior
ListView {
    id: conversationList
    clip: true
    currentIndex: -1

    // conversations and messages are in a single column,
    // this needs to be uninteractive so that they scroll together
    interactive: false
    height: contentHeight

    signal messagePositionRequested(var requestedMsgId)

    // Jump to message on message searched.
    Connections {
        target: sideBarPaneRoot.messageSearchLoader.item

        onMessageClicked: {
            const conv_idx = Herald.conversations.indexById(
                               searchConversationId)

            // early return on out of bounds
            if ((conv_idx < 0) || (conv_idx >= conversationList.count))
                return

            conversationList.currentIndex = conv_idx

            chatView.sourceComponent = conversationList.currentItem.childChatView

            conversationList.messagePositionRequested(searchMsgId)
        }
    }

    delegate: Item {
        id: conversationItem

        readonly property var conversationData: model
        readonly property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise
        property bool outbound: convContent.messages.lastAuthor === Herald.config.configId
        property ConversationContent convContent: ConversationContent {
            conversationId: conversationIdProxy
        }

        property Component childChatView: Component {
            CV.ChatViewMain {
                id: cvMain
                conversationItem: conversationData
                ownedConversation: convContent.messages
                conversationMembers: convContent.members
            }
        }

        visible: conversationData.matched && conversationData.status !== 1
        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        ConversationRectangle {
            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onClicked: {
                    if (mouse.button == Qt.RightButton) {
                        convOptionsMenu.open()
                    } else {
                        chatView.sourceComponent = childChatView
                        conversationList.currentIndex = index
                    }
                }
            }
        }

        Menu {
            id: convOptionsMenu
            MenuItem {
                text: "Mute notifications"
            }

            MenuItem {
                text: "Archive"
                //NOTE: unimplemented for now while backend changes are made, do not uncomment
                //or try to use
                // onTriggered: conversationData.status = 1
            }
        }
    }
}
