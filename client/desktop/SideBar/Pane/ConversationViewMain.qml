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

    //  cacheBuffer: contentHeight * 10
    // conversations and messages are in a single column,
    // this needs to be uninteractive so that they scroll together
    interactive: false
    height: contentHeight
    property bool archiveView: false

    signal messagePositionRequested(var requestedMsgId)

    onContentHeightChanged: if (contentHeight > 0)
                                cacheBuffer = contentHeight * 10

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
        property bool isPairwise: convContent.pairwise
        property bool outbound: convContent.messages.lastAuthor === Herald.config.configId

        property ConversationContent convContent: ContentMap.get(
                                                      conversationIdProxy)

        property int __secondsSinceLastReset: 0
        property int __typing: __secondsSinceLastReset < 8

        property Component childChatView: Component {
            CV.ChatViewMain {
                id: cvMain
                conversationItem: convContent
                ownedConversation: convContent.messages
                conversationMembers: convContent.members
                convId: conversationData.conversationId
            }
        }

        Connections {
            target: convContent.members
            onNewTypingIndicator: {
                conversationItem.__secondsSinceLastReset = 0

                convoRectangle.label.typeActive = true
            }
        }

        Connections {
            target: appRoot.globalTimer
            onRefreshTime: {
                conversationItem.__secondsSinceLastReset += 1
                if (!conversationItem.__typing) {
                    convoRectangle.label.typeActive = false
                }
            }
        }
        Connections {
            target: contactsLoader.item
            onGroupClicked: {

                const groupIdx = Herald.conversations.indexById(groupId)
                if ((groupIdx < 0) || (groupIdx >= conversationList.count))
                    return

                conversationList.currentIndex = groupIdx
                chatView.sourceComponent = conversationList.currentItem.childChatView
                chatView.currentConvoId = groupId
            }
        }

        Connections {
            target: Herald.conversations
            onBuilderConversationIdChanged: {

                if (Herald.conversations.builderConversationId == undefined) {
                    return
                }

                const convIdx = Herald.conversations.indexById(
                                  Herald.conversations.builderConversationId)
                if ((convIdx < 0) || (convIdx >= conversationList.count))
                    return

                conversationList.currentIndex = convIdx
                chatView.sourceComponent = conversationList.currentItem.childChatView
                chatView.currentConvoId = Herald.conversations.builderConversationId
            }
        }

        visible: {
            if (sideBarState.state === "globalSearch") {
                return conversationData.matched
            }
            if (!archiveView) {
                return conversationData.matched && convContent.status !== 1
            }
            return convContent.status === 1
        }

        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        Common.PlatonicRectangle {
            id: convoRectangle
            property bool nts: {
                Herald.utils.compareByteArray(Herald.config.ntsConversationId,
                                              conversationData.conversationId)
            }
            boxTitle: !nts ? convContent.title : qsTr("Note to Self")
            boxColor: !nts ? convContent.conversationColor : UserMap.get(
                                 Herald.config.configId).userColor
            picture: !nts ? Utils.safeStringOrDefault(
                                convContent.picture,
                                "") : Utils.safeStringOrDefault(
                                UserMap.get(
                                    Herald.config.configId).profilePicture, "")
            isGroupPicture: !conversationItem.convContent.pairwise

            labelComponent: Ent.ConversationLabel {
                id: conversationLabel
                width: parent.width
                height: parent.height
                lastMsgDigest: conversationItem.convContent.messages.lastMsgDigest
                isEmpty: {

                    lastMsgDigest == ""
                }
                convoTitle: !convoRectangle.nts ? convContent.title : qsTr(
                                                      "Note to Self")
                labelColor: convoRectangle.state
                            !== "" ? CmnCfg.palette.black : CmnCfg.palette.lightGrey
                minorTextColor: convoRectangle.state
                                !== "" ? CmnCfg.palette.offBlack : CmnCfg.palette.medGrey
                receiptFill: convoRectangle.state
                             !== "" ? CmnCfg.palette.offBlack : CmnCfg.palette.white
                typeColor: Qt.darker(CmnCfg.palette.medGrey, 1.2)
                typeColorAnim: convoRectangle.state
                               !== "" ? CmnCfg.palette.darkGrey : CmnCfg.palette.lightGrey
            }

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                cursorShape: conversationList.currentIndex
                             === index ? Qt.ArrowCursor : Qt.PointingHandCursor
                onClicked: {
                    if (mouse.button === Qt.RightButton) {
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
                text: qsTr("Mute notifications")
            }

            MenuItem {
                text: "Archive"
                onTriggered: {
                    convContent.status = 1

                    const convIdx = Herald.conversations.indexById(
                                      conversationData.conversationId)
                    if ((convIdx < 0) || (convIdx >= conversationList.count))
                        return

                    if (conversationList.currentIndex === convIdx) {

                        conversationList.currentIndex = -1
                        print(conversationList.currentIndex)
                        chatView.sourceComponent = splash
                    }
                }
            }
        }
        Menu {
            id: unarchiveMenu
            MenuItem {
                text: qsTr("Unarchive conversation")
                onTriggered: convContent.status = 0
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
