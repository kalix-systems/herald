import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "../Common"
import "qrc:/imports"
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils

Page {
    id: page
    property var userData
    property var headerComponent: Component {
        ContactInfoHeader {
            contactName: userData !== undefined ? userData.name : ""
        }
    }
    Loader {
        anchors.fill: parent
        active: page.userData !== undefined
        sourceComponent: Flickable {
            boundsBehavior: Flickable.StopAtBounds
            boundsMovement: Flickable.StopAtBounds
            anchors.fill: parent
            contentHeight: wrapperCol.height
            clip: true
            Column {
                id: wrapperCol
                padding: CmnCfg.defaultMargin
                width: parent.width
                spacing: CmnCfg.defaultMargin
                Item {
                    id: contactWrapper
                    anchors.left: parent.left
                    height: CmnCfg.units.dp(60)
                    width: parent.width

                    PlatonicRectangle {
                        id: contactRectangle
                        color: CmnCfg.palette.white
                        boxColor: page.userData.userColor
                        boxTitle: page.userData.name
                        picture: Utils.safeStringOrDefault(
                                     page.userData.profilePicture, "")
                        conversationItemAvatar.size: CmnCfg.units.dp(48)

                        labelComponent: ContactLabel {
                            displayName: page.userData.name
                            username: page.userData.userId
                            labelColor: CmnCfg.palette.black
                            displayNameSize: CmnCfg.labelFontSize
                            usernameSize: CmnCfg.defaultFontSize
                        }
                    }
                }

                Label {
                    id: optionsHeader
                    text: qsTr("Options")
                    font.family: CmnCfg.chatFont.name
                    color: CmnCfg.palette.darkGrey
                    font.pixelSize: CmnCfg.defaultFontSize
                }

                Row {
                    height: implicitHeight

                    spacing: CmnCfg.units.dp(14) //CmnCfg.megaMargin
                    padding: 0
                    Rectangle {
                        id: colorDot
                        height: CmnCfg.units.dp(20)
                        width: height
                        radius: width
                        color: CmnCfg.palette.avatarColors[userData.userColor]
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
                                    font.pixelSize: CmnCfg.chatTextSize
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

                    Label {
                        text: qsTr("Color")
                        font.family: CmnCfg.chatFont.name
                        anchors.verticalCenter: colorDot.verticalCenter
                        font.pixelSize: CmnCfg.chatTextSize
                    }
                }

                Label {
                    id: groupsHeader
                    text: qsTr("Common groups")
                    font.family: CmnCfg.chatFont.name
                    color: CmnCfg.palette.darkGrey
                    visible: groups.count > 0
                    font.pixelSize: CmnCfg.defaultFontSize
                }

                ListView {
                    id: groups
                    model: SharedConversations {
                        id: sharedconvos
                        userId: page.userData.userId
                    }
                    width: parent.width
                    height: contentHeight

                    delegate: Item {
                        width: parent.width
                        property var groupData: model
                        height: 42
                        Avatar {
                            id: groupPic
                            height: 32
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
                                    page.close()
                                    groupClicked(groupData.conversationId)
                                    contactsPopup.close()
                                    contactsLoader.active = false
                                }
                            }
                        }

                        Label {
                            anchors.left: groupPic.right
                            anchors.leftMargin: CmnCfg.smallMargin
                            text: Utils.safeStringOrDefault(
                                      groupData.conversationTitle, "")
                            color: CmnCfg.palette.offBlack
                            font.family: CmnCfg.chatFont.name
                            anchors.verticalCenter: groupPic.verticalCenter
                            font.pixelSize: CmnCfg.chatTextSize
                        }
                    }
                }
            }
        }
    }
}
