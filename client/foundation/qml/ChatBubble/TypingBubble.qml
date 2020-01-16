import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "../Entity"

Rectangle {
    id: bubbleRoot
    property var typingUser: parent.typingUser

    property string typingUserName: Herald.users.nameById(typingUser)
    property color typingUserColor: CmnCfg.palette.avatarColors[Herald.users.userColorById(
                                                                    typingUser)]
    property string typingUserProfilePicture: Herald.users.profilePictureById(
                                                  typingUser)
    property real defaultWidth
    property alias text: actionText.text

    height: CmnCfg.typeMargin
    width: actionText.width + avatar.width + CmnCfg.microMargin

    color: CmnCfg.palette.white

    Rectangle {
        anchors.top: parent.top
        width: parent.width
        height: 1
        color: CmnCfg.palette.medGrey
        z: accent.z + 1
    }

    Rectangle {
        anchors.bottom: parent.bottom
        width: parent.width

        height: 1
        color: CmnCfg.palette.medGrey
        z: accent.z + 1
    }

    Avatar {
        id: avatar
        color: typingUserColor
        initials: typingUserName[0].toUpperCase()
        size: CmnCfg.typeMargin - 8
        anchors {
            left: parent.left
            top: parent.top
            margins: CmnCfg.microMargin
        }

        z: contentRoot.z + 1
        pfpPath: typingUserProfilePicture
    }

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.accentBarWidth
        anchors.left: avatar.right
        anchors.leftMargin: CmnCfg.smallMargin
        visible: false
    }

    Column {
        id: contentRoot
        anchors.left: avatar.right
        anchors.verticalCenter: avatar.verticalCenter

        spacing: CmnCfg.microMargin
        topPadding: CmnCfg.microMargin
        leftPadding: CmnCfg.microMargin
        bottomPadding: CmnCfg.microMargin

        Text {
            id: actionText
            text: typingUserName + " is typing..."
            font.family: CmnCfg.chatFont.name
            font.italic: true
            elide: Text.ElideRight
            width: bubbleRoot.maxWidth
        }
    }
}
