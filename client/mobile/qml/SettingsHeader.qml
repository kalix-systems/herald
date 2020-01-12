import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "Common"

ToolBar {
    height: CmnCfg.toolbarHeight
    width: parent.width

    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }
    AnimIconButton {
        id: drawerButton
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        onTapped: mainView.pop()
        anchors.verticalCenter: parent.verticalCenter
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.defaultMargin
    }

    Label {
        text: qsTr("Settings")
        color: CmnCfg.palette.iconFill
        font.family: CmnCfg.headerFont.family
        font.pixelSize: CmnCfg.headerFontSize
        anchors.verticalCenter: parent.verticalCenter
        anchors.leftMargin: CmnCfg.defaultMargin
        anchors.left: drawerButton.right
    }
}
