import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "qrc:/imports"
import "qrc:/imports/Entity"
import "../../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils

import "../" as Popups

Drawer {
    id: drawer
    property var userData
    width: 0.33 * contactsPopup.width
    height: contactsPopup.height
    edge: Qt.RightEdge
    dragMargin: 0

    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside | Popup.CloseOnReleaseOutside
    Loader {
        anchors.fill: parent
        active: drawer.userData !== undefined
        sourceComponent: Flickable {
            anchors.fill: parent
            Column {
                padding: CmnCfg.defaultMargin
                width: parent.width
                spacing: CmnCfg.defaultMargin
                Item {
                    id: contactWrapper
                    anchors.left: parent.left
                    height: 60
                    width: parent.width
                    Common.PlatonicRectangle {
                        id: contactRectangle
                        color: CmnCfg.palette.white
                        boxColor: drawer.userData.color
                        boxTitle: drawer.userData.name
                        picture: Utils.safeStringOrDefault(
                                     drawer.userData.profilePicture, "")
                        conversationItemAvatar.size: 56

                        //no hover state
                        states: []

                        MouseArea {
                            id: hoverHandler
                        }

                        labelComponent: ContactLabel {
                            displayName: drawer.userData.name
                            username: drawer.userData.userId
                            labelColor: CmnCfg.palette.black
                            displayNameSize: CmnCfg.headerFontSize
                            usernameSize: CmnCfg.defaultFontSize
                        }
                    }
                }

                Label {
                    id: optionsHeader
                    text: qsTr("Options")
                    font.family: CmnCfg.chatFont.name
                    color: CmnCfg.palette.darkGrey
                }

                Row {
                    height: 30

                    spacing: CmnCfg.defaultMargin
                    Rectangle {
                        height: 20
                        width: height
                        radius: width
                        color: CmnCfg.palette.avatarColors[userData.color]
                        MouseArea {
                            id: mouseArea
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                colorLoader.active = true
                                colorLoader.item.open()
                            }
                            ToolTip {
                                visible: mouseArea.containsMouse

                                contentItem: Text {
                                    text: qsTr("Set color")
                                    font.family: CmnCfg.chatFont.name
                                    font.pixelSize: 12
                                }
                                delay: 1000
                                padding: 4
                            }
                        }
                    }

                    Loader {
                        id: colorLoader
                        y: mouse.mouseY
                        active: false

                        sourceComponent: Popups.ColorPicker {

                            id: colorpicker
                            y: mouse.mouseY
                            onClosed: colorLoader.active = false
                            colorCallback: function () {
                                if (userData === undefined)
                                    return
                                var idx = Herald.users.indexById(
                                            userData.userId)

                                if ((idx < 0)
                                        || (idx >= Herald.users.rowCount()))
                                    return

                                Herald.users.setColor(idx, colorIndex)
                            }
                        }
                    }

                    Label {
                        text: qsTr("Color")
                        font.family: CmnCfg.chatFont.name
                    }
                }

                Label {
                    id: groupsHeader
                    text: qsTr("Common groups")
                    font.family: CmnCfg.chatFont.name
                    color: CmnCfg.palette.darkGrey
                }

                ListView {
                    model: SharedConversations {

                        userId: drawer.userData.userId
                    }
                    width: parent.width
                    height: contentHeight

                    delegate: Item {
                        width: parent.width
                        property var groupData: model
                        height: 44
                        Avatar {
                            id: groupPic
                            height: 36
                            isGroup: true

                            property int groupColor: groupData.conversationColor
                                                     !== undefined ? groupData.conversationColor : 0
                            pfpPath: Utils.safeStringOrDefault(
                                         groupData.conversationPicture, "")

                            color: CmnCfg.avatarColors[groupColor]
                            initials: Utils.initialize(
                                          Utils.safeStringOrDefault(
                                              groupData.conversationTitle))
                            MouseArea {
                                anchors.fill: parent
                                cursorShape: Qt.PointingHandCursor
                                hoverEnabled: true
                                onClicked: {

                                    drawer.close()
                                    drawer.modal = false

                                    groupClicked(groupData.conversationId)
                                    contactsPopup.close()
                                    contactsLoader.active = false
                                }
                            }
                        }

                        Label {
                            anchors.left: groupPic.right
                            anchors.leftMargin: CmnCfg.defaultMargin
                            text: Utils.safeStringOrDefault(
                                      groupData.conversationTitle, "")
                            color: CmnCfg.palette.offBlack
                            font.family: CmnCfg.chatFont.name
                            anchors.verticalCenter: groupPic.verticalCenter
                        }
                    }
                }
            }
        }
    }
}
