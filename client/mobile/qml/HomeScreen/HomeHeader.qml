import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import QtQuick 2.14
import LibHerald 1.0
import "../Common"
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import Qt.labs.platform 1.1

ToolBar {
    anchors.fill: parent

    width: parent.width
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }
    Avatar {
        id: avatar
        color: CmnCfg.palette.avatarColors[Herald.config.color]
        initials: Herald.config.name[0].toUpperCase()
        pfpPath: Utils.safeStringOrDefault(Herald.config.profilePicture, "")
        size: CmnCfg.identityAvatarDiameter
        anchors.verticalCenter: parent.verticalCenter
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.defaultMargin
    }

    Label {
        id: stateLabel
        text: qsTr("Conversations")
        font.family: CmnCfg.headerFont.family
        font.pixelSize: CmnCfg.headerFontSize
        anchors.verticalCenter: parent.verticalCenter
        anchors.left: avatar.right
        anchors.leftMargin: CmnCfg.defaultMargin
        color: CmnCfg.palette.iconFill
    }

    Row {
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.right: parent.right
        spacing: CmnCfg.defaultMargin
        layoutDirection: Qt.RightToLeft
        anchors.verticalCenter: parent.verticalCenter
        AnimIconButton {
            id: optionsButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/options-icon.svg"
            onTapped: mainView.push(settingsMain)
        }

        AnimIconButton {
            id: searchButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/search-icon.svg"
            onTapped: mainView.push(globalSearchView)
        }
    }
}
