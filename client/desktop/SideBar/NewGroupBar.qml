import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common

Rectangle {
    width: parent.width
    height: if (visible)
                30
            else
                0

    Common.ButtonForm {
        id: groupIcon
        //when group icon works this will be that instead
        source: "qrc:/add-contact-icon.svg"
        anchors.left: parent.left
        anchors.margins: QmlCfg.margin
        anchors.verticalCenter: parent.verticalCenter

        onClicked: {
            convoPane.state = "newGroupState"

        }
    }

    Text {
        text: "New group"
        anchors.left: groupIcon.right
        anchors.margins: QmlCfg.margin
        anchors.verticalCenter: parent.verticalCenter
    }
}
