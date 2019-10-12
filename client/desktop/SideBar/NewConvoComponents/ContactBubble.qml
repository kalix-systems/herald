import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common

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
        id: innerText
        anchors {
            left: parent.left
            verticalCenter: parent.verticalCenter
            leftMargin: QmlCfg.smallMargin
        }
        color: "white"
        font.bold: true
    }

    Common.ButtonForm {
        id: xButton
        anchors {
            verticalCenter: innerText.verticalCenter
            right: parent.right
            top: parent.top
            margins: QmlCfg.smallMargin
        }
        padding: 0
        source: "qrc:/x-icon-white.svg"
        fill: QmlCfg.palette.iconFill
    }
}
