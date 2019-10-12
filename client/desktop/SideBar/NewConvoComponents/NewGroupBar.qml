import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common

Rectangle {
    id: bgBar
    width: parent.width
    height: visible ? 30 : 0

    property color hoverColor: QmlCfg.palette.secondaryColor
    color: QmlCfg.palette.mainColor

    Common.ButtonForm {
        id: groupIcon
        //when group icon works this will be that instead
        source: "qrc:/add-contact-icon.svg"
        anchors.left: parent.left
        anchors.margins: QmlCfg.margin
        anchors.verticalCenter: parent.verticalCenter
    }

    Text {
        text: "New group"
        anchors.left: groupIcon.right
        anchors.margins: QmlCfg.margin
        anchors.verticalCenter: parent.verticalCenter
        color: QmlCfg.palette.mainTextColor
    }

    Common.Divider {
        anchors.verticalCenter: parent.bottom
        height: 2
        color: QmlCfg.palette.secondaryColor
    }

    MouseArea {
        hoverEnabled: true
        z: 10
        anchors.fill: parent
        // BNOTE: can these be when bindings?
        onEntered: parent.state = "hovering"
        onExited: parent.state = ""

        onClicked: {
            convoPane.state = "newGroupState"
        }
    }

    states: [
        State {
            name: "hovering"
            PropertyChanges {
                target: bgBar
                color: hoverColor
            }
        }
    ]
}
