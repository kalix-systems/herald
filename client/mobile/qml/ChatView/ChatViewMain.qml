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

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    ScrollView {
        id: chatScrollView
        clip: true
        contentWidth: parent.width
        height: chatPage.height - chatTextArea.height
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.smallMargin

        anchors {
            top: parent.top
            right: parent.right
            left: parent.left
        }

        ScrollBar.vertical: ScrollBar {
            id: scrollControl
        }

        TextMessageList {
            messageListModel: ownedMessages
            width: parent.width
            anchors.top: parent.top
        }

        Connections {
            target: ownedMessages
            onRowsInserted: {
                scrollControl.position = 1.0
            }
        }
    }

    Common.Divider {
        width: parent.width
        anchors.bottom: chatTextArea.top
    }

    ChatTextArea {
        id: chatTextArea
        chatName: headerTitle
        property bool risen: false
        anchors {
            right: parent.right
            left: parent.left
            top: chatScrollView.bottom
        }
    }
}
