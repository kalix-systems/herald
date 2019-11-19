import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "qrc:/common" as Common
import "../../../foundation/js/utils.mjs" as Utils
import "../../ChatView" as CV
import ".././js/ContactView.mjs" as JS
import "../popups" as Popups
import "Controls"

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
    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: conversationItem

        readonly property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise
        property Messages messageModel: Messages {
            conversationId: conversationIdProxy
        }
        property var childChatView: Component {
            CV.ChatView {
                conversationAvatar: convoRectangle.conversationItemAvatar
                ownedConversation: messageModel
            }
        }

        Members {
            id: convoItemMembers
            conversationId: conversationIdProxy
        }

        visible: matched
        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        Common.PlatonicRectangle {
            id: convoRectangle
            boxColor: conversationsModel.color(index)
            boxTitle: Utils.unwrapOr(title, "unknown")
            isContact: false

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
