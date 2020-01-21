import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "../../Common"

Rectangle {
    signal deactivate
    signal activate
    property real boundHeight: 0
    onActivate: boundHeight = 50
    onDeactivate: boundHeight = 0

    visible: height != 0
    color: CmnCfg.palette.offBlack
    height: content.height

    property bool active: height > 0
    property bool isArchived: false

    Behavior on height {
        NumberAnimation {
            easing.type: Easing.InOutQuad
            duration: 100
        }
    }

    Connections {
        target: cvMainView
        onCloseAllOptionsBars: {
            if (active)
                deactivate()
                conversationItem.isSelected = false
        }
    }

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
                conversationItem.conversationData.status = 1
                conversationItem.isSelected = false
                deactivate()
            }
            visible: !isArchived
        }

        AnimIconButton {
            imageSource: "qrc:/unarchive-icon.svg"
            anchors {
                right: parent.right
                rightMargin: CmnCfg.defaultMargin
                verticalCenter: parent.verticalCenter
            }
            icon.color: CmnCfg.palette.white
            onTapped: {
                conversationItem.conversationData.status = 0
                conversationItem.isSelected = false
                deactivate()
            }
            visible: isArchived
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
                conversationItem.isSelected = false
                deactivate()
            }
        }
    }
}
