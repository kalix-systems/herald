import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as UTils

Page {
    anchors.fill: parent
    readonly property Component headerComponent: ContactsHeader {}

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

    ContactDrawer {
        id: drawer
        height: parent.height
        width: parent.width * 0.8
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
            width: contactsPopup.width
            height: visible ? row.height + 1 : 0

            property var sharedConvos: SharedConversations {
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
                    pfpPath: Utils.safeStringOrDefault(model.profilePicture, "")
                    color: CmnCfg.avatarColors[model.userColor]
                    initials: Utils.initialize(name)
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
                        //drawer.userData = userData
                        drawer.open()
                    }

                    //contact label
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

                //common groups
                //CommonGroupsFlow {}
            }
        }
    }
}
