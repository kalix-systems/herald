import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "../../Common"

Rectangle {
    //property var cb
    signal deactivate
    signal activate
    property real boundHeight: 0
    visible: height != 0
    color: CmnCfg.palette.offBlack
    onActivate: boundHeight = 50
    onDeactivate: boundHeight = 0
    height: content.height

    property bool active: height > 0

    Behavior on height {
        NumberAnimation {
            easing.type: Easing.InOutQuad
            duration: 100
        }
    }

//    Connections {
//        target: chatList
//        onCloseDropdown: {
//            if (active)
//                deactivate()
//        }
//    }

    Item {
        id: content
        height: boundHeight
        anchors {
            left: parent.left
            right: parent.right
        }
        clip: true

        AnimIconButton {
            imageSource: "qrc:/archive-icon.svg"
            anchors {
                right: parent.right
                rightMargin: CmnCfg.defaultMargin
                verticalCenter: parent.verticalCenter
            }
            icon.color: CmnCfg.palette.white
            onTapped: {
                Herald.conversations.setStatusById(
                            conversationItem.convoContent.conversationId, 1)
                deactivate()
            }
        }

        AnimIconButton {
            icon.color: CmnCfg.palette.white
            imageSource: "qrc:/x-icon.svg"
            anchors {
                left: parent.left
                leftMargin: CmnCfg.defaultMargin
                verticalCenter: parent.verticalCenter
            }
            onTapped: {
                deactivate()
                conversationItem.isSelected = false
            }
        }
    }
}
