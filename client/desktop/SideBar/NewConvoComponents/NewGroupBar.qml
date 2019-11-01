import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common

Rectangle {
    id: bgBar
    width: parent.width
    height: visible ? 30 : 0
    z: 100

    property color hoverColor: CmnCfg.palette.secondaryColor
    color: CmnCfg.palette.mainColor

    Common.ButtonForm {
        id: groupIcon
        //when group icon works this will be that instead
        source: "qrc:/add-contact-icon.svg"
        anchors.left: parent.left
        anchors.margins: CmnCfg.margin
        anchors.verticalCenter: parent.verticalCenter
    }

    Text {
        text: "New group"
        anchors.left: groupIcon.right
        anchors.margins: CmnCfg.margin
        anchors.verticalCenter: parent.verticalCenter
        color: CmnCfg.palette.mainTextColor
    }

    Common.Divider {
        anchors.verticalCenter: parent.bottom
        height: 2
        color: CmnCfg.palette.secondaryColor
    }

    MouseArea {
        id: gBarMouseArea
        hoverEnabled: true
        z: 10
        anchors.fill: parent
        onClicked: {
            sideBarState.state = "newGroupState"
        }
    }

    states: [
        State {
            name: "hovering"
            when: gBarMouseArea.containsMouse
            PropertyChanges {
                target: bgBar
                color: hoverColor
            }
        }
    ]
}
