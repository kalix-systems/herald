import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "qrc:/common" as Common
import "qrc:/imports/Avatar" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "../../ChatView" as CV
import ".././js/ContactView.mjs" as JS
import "../popups" as Popups

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC

/// --- displays a list of conversations
ListView {
    id: conversationList
    clip: true
    currentIndex: -1
    interactive: false
    height: contentHeight

    Connections {
        target: sideBarPaneRoot.messageSearchLoader.item

        onMessageClicked: {
            const conv_idx = herald.conversations.indexById(
                               searchConversationId)
            // early return on out of bounds
            if (conv_idx < 0)
                return

            conversationList.currentIndex = conv_idx
            chatView.sourceComponent = conversationList.currentItem.childChatView
        }
    }

    delegate: Item {
        id: conversationItem

        readonly property var conversationData: model
        readonly property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise
        property bool outbound: convoContent.messages.lastAuthor === herald.config.configId
        property ConversationContent convoContent: ConversationContent {
            conversationId: conversationIdProxy
        }

        property var childChatView: Component {
            CV.ChatViewMain {
                conversationItem: conversationData
                ownedConversation: convoContent.messages
            }
        }

        visible: matched
        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        Common.PlatonicRectangle {
            id: convoRectangle
            boxTitle: title
            boxColor: conversationData.color
            picture: Utils.safeStringOrDefault(conversationData.picture, "")
            groupPicture: !conversationData.pairwise
            //this is in here instead of platonic rectangle bc different for contact and convo
            labelComponent: Av.ConversationLabel {
                contactName: title
                lastBody: !convoContent.messages.isEmpty ? lastAuthor + ": "
                                                           + convoContent.messages.lastBody : ""
                lastAuthor: outbound ? "You" : convoContent.messages.lastAuthor
                lastTimestamp: !convoContent.messages.isEmpty ? Utils.friendlyTimestamp(
                                                                    convoContent.messages.lastTime) : ""
                labelColor: CmnCfg.palette.black
                labelSize: 14
            }

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
                onClicked: {
                    chatView.sourceComponent = childChatView
                    conversationList.currentIndex = index
                }
            }
        }
    }
}
