import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "./Controls"
import "../Common" as Common

Page {
    //swappable message model, set by the appstate
    property Messages ownedMessages
    property string headerTitle

    header: ChatViewHeader {
        title: headerTitle
    }

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    ScrollView {
        id: chatScrollView
        clip: true
        contentWidth: parent.width
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.smallMargin
        ScrollBar.vertical: ScrollBar {
            id: scrollControl
        }

        TextMessageList {
            model: ownedMessages
            width: parent.width
        }

        anchors {
            top: parent.top
            right: parent.right
            left: parent.left
            bottom: chatTextArea.top
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
        anchors {
            bottom: parent.bottom
            right: parent.right
            left: parent.left
        }
    }
}
