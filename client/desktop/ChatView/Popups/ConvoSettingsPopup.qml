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
import "../../SideBar/popups" as SBPopups
import "qrc:/imports/Settings/SettingsComponents" as SC

Popup {
    id: convoSettingsPopup
    property var convoData: parent.convoData
    property var convoMembers: parent.convoMembers

    padding: 0
    height: chatView.height
    width: parent.width
    anchors.centerIn: parent
    onClosed: groupSettingsLoader.active = false
    modal: true

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
            groupSettingsLoader.active = false
        }
        z: header.z + 1
    }

    Rectangle {
        id: header
        anchors.top: parent.top
        anchors.left: parent.left
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

    Flickable {
        width: parent.width
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
            delegate: Column {
                width: parent.width
                spacing: CmnCfg.smallMargin
                leftPadding: CmnCfg.defaultMargin
                property var contactMember: model
                height: visible ? contentHeight : 0
                visible: contactMember.userId !== Herald.config.configId
                Row {
                    spacing: CmnCfg.megaMargin
                    height: 72
                    width: parent.width
                    Entity.Avatar {
                        id: itemAvatar
                        color: CmnCfg.palette.avatarColors[convoData.conversationColor]
                        initials: contactMember.name[0].toUpperCase()
                        pfpPath: Utils.safeStringOrDefault(
                                     contactMember.picture)
                        size: 60
                    }

                    Entity.ContactLabel {
                        anchors.fill: undefined
                        anchors.verticalCenter: itemAvatar.verticalCenter
                        displayNameSize: CmnCfg.headerFontSize
                        width: 60
                        displayName: contactMember.name
                        username: contactMember.userId
                        height: 40
                    }
                }

                Label {
                    id: optionsHeader
                    text: qsTr("Contact settings")
                    font.weight: Font.Medium
                    font.pixelSize: CmnCfg.defaultFontSize
                    font.family: CmnCfg.labelFont.name
                }

                Item {
                    anchors.left: parent.left
                    anchors.right: parent.right

                    anchors.leftMargin: CmnCfg.defaultMargin
                    height: conf.height
                    Label {
                        anchors.left: parent.left

                        font.pixelSize: CmnCfg.chatTextSize
                        text: qsTr("Trusted (share display name and avatar)")
                        font.family: CmnCfg.chatFont.name
                        anchors.verticalCenter: conf.verticalCenter
                    }

                    SC.ConfSwitch {
                        id: conf
                        checked: false
                        anchors.right: parent.right
                        anchors.rightMargin: CmnCfg.defaultMargin
                    }
                }

                Rectangle {
                    anchors.left: parent.left
                    width: parent.width
                    height: 1
                    color: CmnCfg.palette.medGrey
                }

                Item {
                    height: colorDot.height
                    anchors.left: parent.left
                    anchors.leftMargin: CmnCfg.defaultMargin
                    anchors.right: parent.right

                    Label {
                        anchors.left: parent.left

                        font.pixelSize: CmnCfg.chatTextSize
                        text: qsTr("Color")
                        font.family: CmnCfg.chatFont.name
                        anchors.verticalCenter: colorDot.verticalCenter
                    }

                    Rectangle {
                        anchors.right: parent.right
                        anchors.rightMargin: CmnCfg.defaultMargin
                        id: colorDot
                        height: 18
                        width: height
                        radius: width
                        color: CmnCfg.palette.avatarColors[convoData.conversationColor]
                        MouseArea {
                            id: mouseArea
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                colorLoader.active = true
                                colorLoader.item.open()
                            }
                            Loader {
                                id: colorLoader
                                y: mouse.mouseY
                                x: mouse.mouseX - width
                                active: false

                                sourceComponent: SBPopups.ColorPicker {

                                    id: colorpicker
                                    y: mouse.mouseY
                                    x: mouse.mouseX - width + colorDot.width
                                    onClosed: colorLoader.active = false
                                    colorCallback: function () {
                                        if (contactMember === undefined)
                                            return
                                        var idx = Herald.users.indexById(
                                                    contactMember.userId)
                                        if ((idx < 0)
                                                || (idx >= Herald.users.rowCount(
                                                        )))
                                            return

                                        Herald.users.setColor(idx, colorIndex)
                                    }
                                }
                            }
                            ToolTip {
                                visible: mouseArea.containsMouse

                                contentItem: Text {
                                    text: qsTr("Set color")
                                    font.family: CmnCfg.chatFont.name
                                    font.pixelSize: 12
                                    color: CmnCfg.sysPalette.text
                                }
                                background: Rectangle {
                                    color: CmnCfg.sysPalette.window
                                    border.width: 1
                                    border.color: CmnCfg.sysPalette.midlight
                                }
                                delay: 1000
                                padding: CmnCfg.microMargin
                            }
                        }
                    }
                }

                Rectangle {
                    anchors.left: parent.left
                    width: parent.width
                    height: 1
                    color: CmnCfg.palette.medGrey
                }
            }
        }
    }
}
