import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import QtQuick 2.14
import LibHerald 1.0
import "../Common"

ToolBar {

    width: parent.width
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    AnimIconButton {
        id: backButton
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        onTapped: mainView.pop()
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        anchors.bottomMargin: CmnCfg.smallMargin
    }

    Label {
        text: "Message Info"
        color: CmnCfg.palette.iconFill
        anchors.left: backButton.right
        anchors.verticalCenter: parent.verticalCenter
        font.family: CmnCfg.headerFont.family
        font.pixelSize: CmnCfg.headerFontSize
        anchors.leftMargin: CmnCfg.defaultMargin
    }
}
