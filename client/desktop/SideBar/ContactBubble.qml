import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common

Rectangle {
    id: bubble
    property color defaultColor
    property alias text: innerText.text
    property string userId: ""
    property alias xButton: xButton
    width: innerText.width + QmlCfg.margin * 3
    height: innerText.height + QmlCfg.margin
    color: defaultColor
    radius: QmlCfg.radius
    Text {
        anchors.left: parent.left
        anchors.verticalCenter: parent.verticalCenter
        id: innerText
        color: "white"
        font.bold: true
        anchors.leftMargin: QmlCfg.smallMargin
    }

    Common.ButtonForm {
        id: xButton
        anchors.verticalCenter: innerText.verticalCenter
        anchors.right: parent.right
        padding: 0
        scale: 0.6
        source: "qrc:/x-icon-white.svg"
    }

    states: State {
        name: "clickedstate"
        PropertyChanges {
            target: bubble
            color: Qt.lighter(defaultColor, 1.2)
        }
    }
}
