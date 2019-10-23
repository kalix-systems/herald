import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "qrc:/common" as Common
import "qrc:/imports/Avatar" as Av
import "../../../foundation/js/utils.mjs" as Utils
import "../../ChatView" as CV
import ".././js/ContactView.mjs" as JS
import "Controls"
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
    boundsBehavior: Flickable.StopAtBounds

    //PAUL: , lets write our own QML formatter so that this is a one liner
    ScrollBar.vertical: ScrollBar {}

    delegate: Item {
        id: conversationItem

        readonly property var conversationData: model
        readonly property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise

        property Messages messageModel: Messages {
            conversationId: conversationIdProxy
        }

        property var childChatView: Component {
            CV.ChatView {
                conversationItem: conversationData
                ownedConversation: messageModel
            }
        }

        visible: matched
        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        Members {
            id: convoItemMembers
            conversationId: conversationIdProxy
        }

        Common.PlatonicRectangle {
            id: convoRectangle
            boxColor: conversationsModel.color(index)
            boxTitle: Utils.unwrapOr(title, "unknown")
            isContact: false

            ConversationLabel {
                anchors.left: parent.conversationItemAvatar.right
                anchors.right: parent.right
                label: parent.boxTitle
                summaryText: JS.formatSummary(messageModel.lastAuthor,
                                              messageModel.lastBody)
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
