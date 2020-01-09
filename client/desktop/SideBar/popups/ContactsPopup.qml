import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "qrc:/imports"
import "qrc:/imports/Entity"
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "ContactsPopup"
import QtGraphicalEffects 1.0

Popup {
    id: contactsPopup

    height: root.height
    width: root.width
    anchors.centerIn: parent
    onClosed: {
        drawer.close()
        contactsLoader.active = false
    }
    padding: 0

    signal groupClicked(var groupId)

    ContactDrawer {
        id: drawer
    }

    Page {
        anchors.fill: parent
        header: ToolBar {
            id: toolBar
            height: CmnCfg.toolbarHeight + 1
            width: parent.width
            background: Rectangle {
                color: CmnCfg.palette.offBlack
            }

            Label {
                font: CmnCfg.headerFont
                anchors.left: parent.left
                anchors.leftMargin: CmnCfg.megaMargin
                anchors.verticalCenter: parent.verticalCenter
                elide: Label.ElideRight
                text: "Contacts"
                color: CmnCfg.palette.white
                topPadding: 1
            }
            Row {
                anchors.verticalCenter: parent.verticalCenter
                anchors.right: parent.right
                anchors.rightMargin: CmnCfg.defaultMargin
                spacing: CmnCfg.defaultMargin
                layoutDirection: Qt.RightToLeft
                IconButton {
                    id: xButton
                    fill: CmnCfg.palette.lightGrey
                    source: "qrc:/x-icon.svg"
                    enabled: !drawer.opened
                    onClicked: {
                        contactsPopup.close()
                    }
                }

                IconButton {
                    id: settingsButton
                    fill: CmnCfg.palette.lightGrey
                    source: "qrc:/options-icon.svg"
                    enabled: !drawer.opened
                }
                IconButton {
                    id: searchButton
                    fill: CmnCfg.palette.lightGrey
                    source: "qrc:/search-icon.svg"
                    enabled: !drawer.opened
                }
            }
        }

        Item {
            id: rowLabel
            height: CmnCfg.toolbarHeight - 10
            width: parent.width

            Item {
                width: CmnCfg.avatarSize
                anchors.left: parent.left
                id: avatarFiller
                anchors.leftMargin: CmnCfg.defaultMargin
            }

            Text {
                id: nameHeader
                anchors.left: avatarFiller.right
                anchors.leftMargin: CmnCfg.megaMargin
                text: "Name"
                anchors.verticalCenter: parent.verticalCenter
                font.family: CmnCfg.chatFont.name
                color: CmnCfg.palette.offBlack
                font.pixelSize: CmnCfg.defaultFontSize
                font.weight: Font.Medium
            }

            Text {
                text: "Groups"
                anchors.verticalCenter: parent.verticalCenter
                anchors.horizontalCenter: parent.horizontalCenter
                font.family: CmnCfg.chatFont.name
                color: CmnCfg.palette.offBlack
                font.pixelSize: CmnCfg.defaultFontSize
                font.weight: Font.Medium
            }
        }

        ListView {
            id: tableView
            boundsBehavior: Flickable.StopAtBounds
            boundsMovement: Flickable.StopAtBounds
            anchors {
                top: rowLabel.bottom
                bottom: parent.bottom
                right: parent.right
                left: parent.left
            }
            model: Herald.users
            delegate: Rectangle {
                id: userRow
                property var userData: model
                color: CmnCfg.palette.white
                width: contactsPopup.width
                height: visible ? row.height + 1 : 0

                property var sharedConvos: SharedConversations {
                    userId: userData.userId
                }

                visible: userData.userId !== Herald.config.configId
                Rectangle {
                    anchors {
                        right: parent.right
                        left: parent.left
                        top: parent.top
                    }
                    height: 1
                    color: CmnCfg.palette.medGrey
                }

                Item {
                    id: row
                    width: contactsPopup.width
                    height: 70
                    Avatar {
                        id: avatar
                        anchors.left: parent.left
                        anchors.leftMargin: CmnCfg.defaultMargin
                        anchors.verticalCenter: parent.verticalCenter
                        height: CmnCfg.avatarSize
                        pfpPath: Utils.safeStringOrDefault(
                                     model.profilePicture, "")
                        color: CmnCfg.avatarColors[model.color]
                        initials: Utils.initialize(name)
                    }

                    MouseArea {
                        height: labelCol.height
                        width: labelCol.width
                        cursorShape: Qt.PointingHandCursor
                        anchors.left: avatar.right
                        anchors.leftMargin: CmnCfg.megaMargin
                        anchors.verticalCenter: avatar.verticalCenter
                        onClicked: {
                            drawer.userData = userData
                            drawer.open()
                        }

                        Column {
                            id: labelCol
                            spacing: 2
                            Label {
                                font.weight: Font.DemiBold
                                font.pixelSize: CmnCfg.headerFontSize
                                font.family: CmnCfg.chatFont.name
                                text: userId
                                color: CmnCfg.palette.offBlack
                            }
                            Label {
                                text: "@" + name
                                font.family: CmnCfg.chatFont.name
                                color: CmnCfg.palette.offBlack
                                font.pixelSize: CmnCfg.defaultFontSize
                            }
                        }
                    }

                    Flow {
                        anchors.horizontalCenter: parent.horizontalCenter
                        anchors.verticalCenter: parent.verticalCenter
                        spacing: CmnCfg.microMargin
                        width: 100

                        Repeater {
                            model: userRow.sharedConvos
                            delegate: Avatar {
                                id: groupAv
                                property var groupData: model
                                height: 30
                                isGroup: true
                                visible: index < 6

                                property int groupColor: groupData.conversationColor !== undefined ? groupData.conversationColor : 0
                                pfpPath: Utils.safeStringOrDefault(
                                             groupData.conversationPicture, "")

                                color: CmnCfg.avatarColors[groupColor]
                                initials: Utils.initialize(
                                              Utils.safeStringOrDefault(
                                                  groupData.conversationTitle))

                                MouseArea {
                                    enabled: !overlay.visible
                                    anchors.fill: parent
                                    cursorShape: Qt.PointingHandCursor
                                    hoverEnabled: true
                                    onClicked: {
                                        groupClicked(groupData.conversationId)
                                        contactsPopup.close()
                                        contactsLoader.active = false
                                    }
                                    ToolTip {
                                        visible: parent.containsMouse
                                        contentItem: Text {
                                            text: Utils.safeStringOrDefault(
                                                      groupData.conversationTitle,
                                                      "")
                                            font.family: CmnCfg.chatFont.name
                                            font.pixelSize: 12
                                            color: CmnCfg.palette.lightGrey
                                            font.weight: Font.Medium
                                        }
                                        delay: 1000
                                        padding: 4
                                        background: Rectangle {
                                            color: CmnCfg.palette.offBlack
                                        }
                                    }
                                }
                                Rectangle {
                                    anchors.fill: parent
                                    color: "transparent"

                                    id: overlay
                                    visible: (userRow.sharedConvos.rowCount(
                                                  ) > 5 && index === 5)
                                    ColorOverlay {
                                        anchors.fill: parent
                                        color: "black"
                                        opacity: 0.5
                                    }
                                    MouseArea {
                                        anchors.fill: parent
                                        preventStealing: true
                                        propagateComposedEvents: false
                                        z: groupAv.z + 1
                                        hoverEnabled: false
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            drawer.userData = userData
                                            drawer.open()
                                        }
                                    }

                                    Label {
                                        anchors.centerIn: parent
                                        text: "+" + (userRow.sharedConvos.rowCount(
                                                         ) - 6)
                                        color: "white"
                                        font.family: CmnCfg.chatFont.name
                                        font.weight: Font.DemiBold
                                        font.pixelSize: CmnCfg.defaultFontSize
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Rectangle {
            anchors {
                right: parent.right
                left: parent.left
                top: tableView.bottom
            }
            height: 1
            color: CmnCfg.palette.offBlack
        }
    }
}
