import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import QtQuick 2.14
import LibHerald 1.0
import Qt.labs.platform 1.1
import "../Common"
import "./Controls" as Controls
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils

ToolBar {
    property var parentPage

    anchors.fill: parent

    width: parent.width
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    AnimIconButton {
        id: backButton
        imageSource: "qrc:/back-arrow-icon.svg"
        visible: parentPage.state === "archiveState"
        color: CmnCfg.palette.iconFill
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        anchors.bottomMargin: CmnCfg.units.dp(6)
        onTapped: {
            cvMainView.state = "default"
        }
    }

    Avatar {
        id: avatar
        color: CmnCfg.palette.avatarColors[Herald.config.configColor]
        initials: Herald.config.name[0].toUpperCase()
        pfpPath: Utils.safeStringOrDefault(Herald.config.profilePicture, "")
        size: CmnCfg.headerAvatarSize
        visible: parentPage.state !== "archiveState"
        anchors.verticalCenter: parent.verticalCenter
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.defaultMargin
    }

    GridLayout {
        id: stateGrid
        anchors.left: avatar.right
        anchors.leftMargin: CmnCfg.defaultMargin
        anchors.bottom: parent.bottom
        anchors.bottomMargin: CmnCfg.units.dp(2)

        Label {
            id: stateLabel
            text: parentPage.state === "archiveState" ? qsTr("Archived") :
                                                        qsTr("Conversations")
            font.family: CmnCfg.headerFont.family
            font.pixelSize: CmnCfg.headerFontSize
            Layout.maximumWidth: parent.width - avatar.width - buttonRow.implicitWidth
            color: CmnCfg.palette.iconFill
        }
    }

    Row {
        id: buttonRow
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.right: parent.right
        spacing: CmnCfg.defaultMargin
        anchors.verticalCenter: parent.verticalCenter

        visible: parentPage.state !== "archiveState"

        AnimIconButton {
            id: searchButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/search-icon.svg"
            onTapped: mainView.push(globalSearchView, StackView.Immediate)
        }

        AnimIconButton {
            id: contactsButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/directory-icon.svg"
            onTapped: mainView.push(contactsViewMain)
        }

        AnimIconButton {
            id: optionsButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/options-icon.svg"
            onTapped: optionsMenu.open()
        }
    }

    Controls.OptionsMenu {
        id: optionsMenu
    }
}
