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

        //header
        header: ToolBar {
            id: toolBar
            height: CmnCfg.toolbarHeight + 1
            width: parent.width
            background: Rectangle {
                color: CmnCfg.palette.offBlack
            }

            Label {
                id: headerLabel
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
                height: parent.height
                anchors.verticalCenter: parent.verticalCenter
                anchors.right: parent.right
                anchors.rightMargin: CmnCfg.defaultMargin
                spacing: CmnCfg.defaultMargin
                layoutDirection: Qt.RightToLeft
                IconButton {
                    id: xButton
                    fill: CmnCfg.palette.lightGrey
                    anchors.verticalCenter: parent.verticalCenter
                    source: "qrc:/x-icon.svg"
                    onClicked: {
                        contactsPopup.close()
                    }
                }

                RowLayout {
                    spacing: 0
                    anchors.verticalCenter: parent.verticalCenter
                    height: parent.height
                    id: searchRow
                    IconButton {
                        id: searchButton
                        property bool search: false
                        fill: CmnCfg.palette.lightGrey
                        source: "qrc:/search-icon.svg"
                        onClicked: search = true
                    }

                    BorderedTextField {
                        id: field
                        placeholderText: "Search names, groups"
                        visible: searchButton.search
                        Layout.maximumWidth: contactsPopup.width - 250
                        selectByMouse: true
                        Layout.bottomMargin: CmnCfg.smallMargin
                        Layout.leftMargin: CmnCfg.smallMargin
                        borderColor: "transparent"
                        onTextChanged: {
                            Qt.callLater(function (text) {
                                Herald.users.filter = text
                            }, field.text)
                        }
                    }
                    Item {
                        visible: searchButton.search
                        height: field.height
                        width: searchX.width
                        Layout.bottomMargin: CmnCfg.smallMargin
                        IconButton {
                            id: searchX
                            anchors.left: parent.left
                            anchors.leftMargin: 2
                            anchors.bottom: divider.top
                            fill: CmnCfg.palette.lightGrey
                            source: "qrc:/x-icon.svg"
                            scale: 0.8
                            onClicked: {
                                searchButton.search = !searchButton.search
                                field.text = ""
                                Herald.users.filter = ""
                            }
                        }
                        Rectangle {
                            id: divider
                            anchors.bottom: parent.bottom
                            anchors.right: parent.right
                            width: searchRow.width - searchButton.width - CmnCfg.units.dp(
                                       6)
                            color: CmnCfg.palette.lightGrey
                            height: 1
                        }
                    }
                }
            }
        }

        background: Rectangle {
            color: CmnCfg.palette.lightGrey
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
                anchors.right: groupHeader.left
                anchors.rightMargin: CmnCfg.megaMargin
            }

            Text {
                text: "Groups"
                id: groupHeader
                anchors.verticalCenter: parent.verticalCenter
                anchors.horizontalCenter: parent.horizontalCenter
                font.family: CmnCfg.chatFont.name
                color: CmnCfg.palette.offBlack
                font.pixelSize: CmnCfg.defaultFontSize
                font.weight: Font.Medium
            }
            Rectangle {
                anchors {
                    right: parent.right
                    left: parent.left
                    bottom: parent.bottom
                }
                height: 1
                color: CmnCfg.palette.medGrey
            }
        }

        //contacts list view
        ListView {
            id: listView
            boundsBehavior: Flickable.StopAtBounds
            boundsMovement: Flickable.StopAtBounds
            anchors {
                top: rowLabel.bottom
                right: parent.right
                left: parent.left
                bottom: parent.bottom
            }
            width: parent.width

            clip: true
            maximumFlickVelocity: 1500
            flickDeceleration: listView.height * 10
            contentWidth: width
            model: Herald.users
            ScrollBar.vertical: ScrollBar {}

            delegate: Rectangle {
                id: userRect
                property User userData: UserMap.get(model.userId)
                color: CmnCfg.palette.white
                width: contactsPopup.width
                height: visible ? row.height + 1 : 0

                property SharedConversations sharedConvos: SharedConversations {
                    userId: userData.userId
                }

                visible: (userData.userId !== Herald.config.configId && matched)

                //top header
                Rectangle {
                    anchors {
                        right: parent.right
                        left: parent.left
                        top: parent.top
                    }
                    height: 1
                    visible: index !== 0
                    color: CmnCfg.palette.medGrey
                }

                //bottom header
                Rectangle {
                    anchors {
                        right: parent.right
                        left: parent.left
                        bottom: parent.bottom
                    }
                    height: 1
                    color: CmnCfg.palette.medGrey
                    z: parent.z + 1
                    visible: index === (listView.count - 1)
                }

                //item wrapping avatar and label; not using platonic rectangle
                //so they can have separate mouse areas
                Item {
                    id: row
                    width: contactsPopup.width
                    height: 70

                    //avatar
                    Avatar {
                        id: avatar
                        anchors.left: parent.left
                        anchors.leftMargin: CmnCfg.defaultMargin
                        anchors.verticalCenter: parent.verticalCenter
                        height: CmnCfg.avatarSize
                        pfpPath: Utils.safeStringOrDefault(
                                     userData.profilePicture, "")
                        color: CmnCfg.avatarColors[userData.userColor]
                        initials: Utils.initialize(userData.name)
                        MouseArea {
                            cursorShape: Qt.PointingHandCursor
                            anchors.fill: parent
                            onClicked: {
                                drawer.userData = userData
                                drawer.open()
                            }
                        }
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

                        //contact label
                        Column {
                            id: labelCol
                            spacing: 2
                            width: row.width
                            GridLayout {
                                width: labelCol.width
                                Label {
                                    font.weight: Font.DemiBold
                                    font.pixelSize: CmnCfg.labelFontSize
                                    font.family: CmnCfg.chatFont.name
                                    text: userData.name
                                    color: CmnCfg.palette.offBlack
                                    Layout.maximumWidth: nameHeader.width
                                    elide: Label.ElideRight
                                }
                            }
                            Label {
                                text: "@" + userData.userId
                                font.family: CmnCfg.chatFont.name
                                color: CmnCfg.palette.offBlack
                                font.pixelSize: CmnCfg.defaultFontSize
                            }
                        }
                    }

                    //common groups
                    CommonGroupsFlow {
                        id: groups
                    }
                }
            }
        }
    }
}
