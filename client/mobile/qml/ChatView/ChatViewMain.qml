import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../Common" as Common
import "qrc:/imports/ChatBubble" as CB

Page {
    id: chatPage
    readonly property Component headerComponent: ChatHeader {}
    //swappable message model, set by the appstate
    property Messages ownedMessages: convContent !== undefined ? convContent.messages : undefined
    property string headerTitle: convContent !== undefined ? convContent.title : ""
    property ConversationContent convContent: ContentMap.get(convId)
    property var convId

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    ChatListView {
        id: chatList
        clip: true
        messageListModel: ownedMessages
        width: parent.width
        anchors.top: parent.top
        anchors.bottom: divider.top
    }

    Connections {
        target: appRouter
        onMessagePosRequested: {
            const msg_idx = ownedMessages.indexById(requestedMsgId)
            // early return on out of bounds
            if ((msg_idx < 0) || (msg_idx >= chatList.count))
                return

            chatList.positionViewAtIndex(msg_idx, ListView.Center)
            chatList.highlightAnimation.target = chatList.itemAtIndex(
                        msg_idx).highlightItem
            chatList.highlightAnimation.start()
        }
    }
    CB.TypingBubble {
        id: typingIndicator
        conversationMembers: convContent.members
        anchors.bottom: divider.top
    }

    Common.Divider {
        width: parent.width
        id: divider
        anchors.bottom: chatTextArea.top
    }

    ChatTextArea {
        id: chatTextArea
        chatName: headerTitle
        property bool risen: false
        anchors {
            right: parent.right
            left: parent.left
            bottom: parent.bottom
        }

        onHeightChanged: {
            if (chatList.height > chatList.contentHeight) {
                chatList.height = chatList.contentHeight
            }
        }
    }
}
