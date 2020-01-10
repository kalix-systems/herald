import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "qrc:/common" as Common
import "qrc:/imports/Entity" as Ent
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
    property bool archiveView: false

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

        Connections {
            target: contactsLoader.item
            onGroupClicked: {

                const groupIdx = Herald.conversations.indexById(groupId)
                if ((groupIdx < 0) || (groupIdx >= conversationList.count))
                    return

                //conversationItem.convContent.conversationId = groupId
                conversationList.currentIndex = groupIdx
                chatView.sourceComponent = conversationList.currentItem.childChatView
                chatView.currentConvoId = groupId
            }
        }
        visible: {
            if (sideBarState.state === "globalSearch") {
                return conversationData.matched
            }
            if (!archiveView) {
                return conversationData.matched && conversationData.status !== 1
            }
            return conversationData.status === 1
        }

        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        Common.PlatonicRectangle {
            id: convoRectangle
            property bool nts: {
                Herald.utils.compareByteArray(Herald.config.ntsConversationId,
                                              conversationData.conversationId)
            }
            boxTitle: !nts ? title : qsTr("Note to Self")
            boxColor: !nts ? conversationData.color : Herald.config.color
            picture: !nts ? Utils.safeStringOrDefault(
                                conversationData.picture,
                                "") : Utils.safeStringOrDefault(
                                Herald.config.profilePicture, "")
            isGroupPicture: !conversationData.pairwise

            labelComponent: Ent.ConversationLabel {
                id: conversationLabel
                lastMsgDigest: conversationItem.conversationData.lastMsgDigest
                isEmpty: conversationItem.conversationData.isEmpty
                convoTitle: !convoRectangle.nts ? title : qsTr("Note to Self")
                labelColor: convoRectangle.state
                            !== "" ? CmnCfg.palette.black : CmnCfg.palette.lightGrey
                minorTextColor: convoRectangle.state
                                !== "" ? CmnCfg.palette.offBlack : CmnCfg.palette.medGrey
            }

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onClicked: {
                    if (mouse.button == Qt.RightButton) {
                        !archiveView ? convOptionsMenu.open(
                                           ) : unarchiveMenu.open()
                    } else {
                        chatView.sourceComponent = childChatView
                        chatView.currentConvoId = conversationData.conversationId
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
                onTriggered: {
                    conversationData.status = 1
                    //                    if (Herald.utils.compareByteArray(
                    //                                chatView.currentConvoId,
                    //                                conversationData.conversationId)) {

                    //                        conversationList.currentIndex = -1
                    //                        chatView.sourceComponent = splash
                    //                        chatView.currentConvoId = undefined
                    //                    }
                }
            }
        }
        Menu {
            id: unarchiveMenu
            MenuItem {
                text: "Unarchive conversation"
                onTriggered: conversationData.status = 0
            }
        }
    }

    states: State {
        name: "archivestate"
        PropertyChanges {
            target: conversationList
            archiveView: true
        }
    }
}
