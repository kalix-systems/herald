import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../Common" as Common

Page {
    id: chatPage
    readonly property Component headerComponent: ChatHeader {}
    //swappable message model, set by the appstate
    property Messages ownedMessages
    property string headerTitle
    property var convoItem

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
    }
}
