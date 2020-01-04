import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports" as Imports
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Entity
import "qrc:/imports/js/utils.mjs" as Utils
import QtQuick.Layouts 1.3
import QtQuick.Shapes 1.12

Popup {
    id: convoSettingsPopup
    property var convoData: parent.convoData
    property var convoMembers: parent.convoMembers

    padding: 0
    height: chatView.height
    width: chatView.width
    anchors.centerIn: parent
    onClosed: groupSettingsLoader.active = false

    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }

    Imports.IconButton {
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.verticalCenter: header.verticalCenter
        icon.source: "qrc:/x-icon.svg"
        fill: CmnCfg.palette.white
        onClicked: {
            convoSettingsPopup.close()
            convoSettingsLoader.active = false
        }
        z: header.z + 1
    }

    Rectangle {
        id: header
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.leftMargin: 1
        anchors.right: parent.right
        height: CmnCfg.toolbarHeight + 1
        color: CmnCfg.palette.offBlack
        Label {
            id: headerLabel
            anchors.left: parent.left
            anchors.leftMargin: CmnCfg.smallMargin
            text: "Conversation settings"
            font.pixelSize: CmnCfg.headerFontSize
            color: CmnCfg.palette.white
            anchors.verticalCenter: parent.verticalCenter
            font.family: CmnCfg.labelFont.name
        }
    }
    Rectangle {
        anchors.right: header.left
        color: CmnCfg.palette.lightGrey
        width: 1
        height: CmnCfg.palette.toolbarHeight
    }

    Flickable {
        width: chatView.width
        anchors.top: header.bottom
        anchors.bottom: parent.bottom
        contentWidth: width
        contentHeight: contactList.height
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        anchors.topMargin: CmnCfg.smallMargin
        ListView {
            id: contactList
            height: 60
            width: parent.width
            model: convoMembers
            delegate: Item {
                width: parent.width
                property var contactMember: model
                height: visible ? 60 : 0
                visible: contactMember.userId !== Herald.config.configId
                Entity.Avatar {
                    id: itemAvatar
                    anchors {
                        left: parent.left
                        verticalCenter: parent.verticalCenter
                        leftMargin: CmnCfg.smallMargin
                    }
                    color: CmnCfg.avatarColors[contactMember.color]
                    initials: contactMember.name[0].toUpperCase()
                    pfpPath: Utils.safeStringOrDefault(contactMember.picture)
                    size: 60
                }

                Entity.ContactLabel {
                    anchors.left: itemAvatar.right
                    anchors.leftMargin: CmnCfg.megaMargin
                    anchors.fill: undefined
                    anchors.verticalCenter: itemAvatar.verticalCenter
                    displayNameSize: CmnCfg.headerFontSize
                    width: 60
                    displayName: contactMember.name
                    username: contactMember.userId
                    height: 40
                }
            }
        }
    }
}
