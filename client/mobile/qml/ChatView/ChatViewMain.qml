import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "./Controls"

Page {
    //swappable message model, set by the appstate
    property Messages ownedMessages
    property string headerTitle

    header: ChatViewHeader {
        title: headerTitle
    }

    background: Rectangle {
        color: CmnCfg.palette.mainColor
    }

    TextMessageList {
        model: ownedMessages
        anchors {
            top: parent.top
            right: parent.right
            left: parent.left
            bottom: chatTextArea.top
        }
    }

    ChatTextArea {
        id: chatTextArea
        anchors {
            bottom: parent.bottom
            right: parent.right
            left: parent.left
        }
    }

    Rectangle {
        anchors.fill: chatTextArea
        color: CmnCfg.palette.secondaryColor
        z: -1
    }
}
