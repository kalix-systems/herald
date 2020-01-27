import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.2
import LibHerald 1.0
import "qrc:/imports/Entity" as Entity
import "qrc:/imports/js/utils.mjs" as Utils
import "../Common" as Common

Page {
    id: contactsView
    readonly property Component headerComponent: ContactsHeader {}
    ContactPage {
        id: contactPage
    }

    Component.onCompleted: appRouter.contactView = contactsView
    signal groupClicked(var groupId)


    Item {
        id: columnLabels
        height: CmnCfg.units.dp(CmnCfg.toolbarHeight - 20)
        width: parent.width

        Text {
            id: nameHeader
            anchors {
                left: parent.left
                // leftMargin calculated from avatar width + margins, plus an
                // infuriatingly mysterious 4dp
                leftMargin: CmnCfg.smallMargin + CmnCfg.avatarSize +
                            CmnCfg.defaultMargin + CmnCfg.units.dp(4)
                right: groupHeader.left
                rightMargin: CmnCfg.megaMargin
                verticalCenter: parent.verticalCenter
            }
            text: "Name"
            color: CmnCfg.palette.offBlack
            font.family: CmnCfg.chatFont.name
            font.pixelSize: CmnCfg.chatTextSize
            font.weight: Font.Medium
        }

        Text {
            id: groupHeader
            text: "Groups"
            anchors.left: parent.left
            // leftMargin calculated from avatarAndName width + left margin
            anchors.leftMargin: CmnCfg.smallMargin + contactsView.width * 0.6
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
            top: columnLabels.bottom
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
            property var userData: UserMap.get(model.userId)
            color: CmnCfg.palette.white
            width: parent.width
            height: visible ? CmnCfg.convoHeight + 1 : 0

            visible: (userData.userId !== Herald.config.configId && matched)

            property var sharedConvos: SharedConversations {
                userId: userData.userId
            }

            // item wrapping avatar and label; not using platonic rectangle
            // because it expects to fill its parent's full width
            Item {
                id: avatarAndName
                width: contactsView.width * 0.6
                height: parent.height

                Entity.Avatar {
                    id: avatar
                    anchors.left: parent.left
                    anchors.leftMargin: CmnCfg.smallMargin
                    anchors.verticalCenter: parent.verticalCenter
                    height: CmnCfg.units.dp(CmnCfg.avatarSize - 10)
                    pfpPath: Utils.safeStringOrDefault(
                                 userData.profilePicture, "")
                    color: CmnCfg.avatarColors[userData.userColor]
                    initials: Utils.initialize(userData.name)
                }

                Entity.ContactLabel {
                    displayName: userData.name
                    username: userId
                    anchors {
                        left: avatar.right
                        leftMargin: CmnCfg.defaultMargin
                        verticalCenter: parent.verticalCenter
                    }
                }

                TapHandler {
                    onTapped: {
                        contactPage.userData = userRect.userData
                        stackView.push(contactPage)
                    }
                }

            }

            CommonGroupsFlow {
                anchors {
                    left: avatarAndName.right
                    leftMargin: CmnCfg.smallMargin
                    right: parent.right
                    rightMargin: CmnCfg.smallMargin
                    verticalCenter: parent.verticalCenter
                }
            }

            //top divider
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

            //bottom divider
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
        }
    }
}
