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
    width: innerText.width + CmnCfg.margin * 3
    height: innerText.height + CmnCfg.margin
    color: defaultColor
    radius: CmnCfg.radius

    Text {
        id: innerText
        anchors {
            left: parent.left
            verticalCenter: parent.verticalCenter
            leftMargin: CmnCfg.smallMargin
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
            margins: CmnCfg.smallMargin
        }
        padding: 0
        source: "qrc:/x-icon-white.svg"
        fill: CmnCfg.palette.iconFill
    }
}
