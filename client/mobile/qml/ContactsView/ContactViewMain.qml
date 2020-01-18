import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import QtQuick.Layouts 1.2

Page {
    id: contactsPopup
    readonly property Component headerComponent: ContactsHeader {}
    ContactPage {
        id: contactPage
    }

    Component.onCompleted: appRouter.contactView = contactsPopup
    signal groupClicked(var groupId)
    Item {
        id: rowLabel
        height: CmnCfg.units.dp(CmnCfg.toolbarHeight - 10)
        width: parent.width

        Item {
            width: CmnCfg.units.dp(CmnCfg.avatarSize - 10)
            anchors.left: parent.left
            id: avatarFiller
            height: CmnCfg.units.dp(10)
            anchors.leftMargin: CmnCfg.smallMargin
        }

        Text {
            id: nameHeader
            anchors.left: avatarFiller.right
            anchors.leftMargin: CmnCfg.smallMargin
            text: "Name"
            anchors.verticalCenter: parent.verticalCenter
            font.family: CmnCfg.chatFont.name
            color: CmnCfg.palette.offBlack
            font.pixelSize: CmnCfg.chatTextSize
            font.weight: Font.Medium
            anchors.right: groupHeader.left
            anchors.rightMargin: CmnCfg.megaMargin
        }

        Text {
            id: groupHeader
            text: "Groups"
            anchors.verticalCenter: parent.verticalCenter
            anchors.horizontalCenter: parent.horizontalCenter
            font.family: CmnCfg.chatFont.name
            color: CmnCfg.palette.offBlack
            font.pixelSize: CmnCfg.chatTextSize
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
            property var userData: model
            color: CmnCfg.palette.white
            width: parent.width
            height: visible ? row.height + 1 : 0

            visible: (userData.userId !== Herald.config.configId && matched)

            property var sharedConvos: SharedConversations {
                userId: userData.userId
            }
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
                height: CmnCfg.units.dp(48)

                //avatar
                Avatar {
                    id: avatar
                    anchors.left: parent.left
                    anchors.leftMargin: CmnCfg.smallMargin
                    anchors.verticalCenter: parent.verticalCenter
                    height: CmnCfg.units.dp(CmnCfg.avatarSize - 10)
                    pfpPath: Utils.safeStringOrDefault(model.profilePicture, "")
                    color: CmnCfg.avatarColors[model.userColor]
                    initials: Utils.initialize(name)
                    MouseArea {
                        cursorShape: Qt.PointingHandCursor
                        anchors.fill: parent
                        onClicked: {
                            contactPage.userData = userRect.userData
                            stackView.push(contactPage)
                        }
                    }
                }
                MouseArea {
                    height: labelCol.height
                    width: labelCol.width
                    cursorShape: Qt.PointingHandCursor
                    anchors.left: avatar.right
                    anchors.leftMargin: CmnCfg.defaultMargin
                    anchors.verticalCenter: avatar.verticalCenter
                    onClicked: {
                        contactPage.userData = userRect.userData
                        stackView.push(contactPage)
                    }

                    //contact label
                    Column {
                        id: labelCol
                        spacing: 2
                        GridLayout {
                            Label {
                                font.weight: Font.DemiBold
                                font.pixelSize: CmnCfg.entityLabelSize
                                font.family: CmnCfg.chatFont.name
                                text: name
                                color: CmnCfg.palette.offBlack
                                Layout.maximumWidth: nameHeader.width
                                elide: Label.ElideRight
                            }
                        }
                        Label {
                            text: "@" + userId
                            font.family: CmnCfg.chatFont.name
                            color: CmnCfg.palette.offBlack
                            font.pixelSize: CmnCfg.entitySubLabelSize
                        }
                    }
                }
                //common groups
                CommonGroupsFlow {}
            }
        }
    }
}
