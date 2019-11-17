import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "../SideBar" as SideBar
import "qrc:/imports/Avatar"

// Shared rectangle for displaying contact and conversation items in sidebar
Rectangle {
    property alias conversationItemAvatar: conversationItemAvatar
    id: bgBox
    color: CmnCfg.palette.paneColor
    anchors.fill: parent
    property string boxTitle
    property int boxColor
    property alias labelComponent: conversationItemAvatar.labelComponent
    property string picture
    property bool groupPicture: false

    AvatarMain {
        anchors.fill: parent
        id: conversationItemAvatar
        iconColor: CmnCfg.avatarColors[boxColor]
        initials: boxTitle[0].toUpperCase()
        pfpPath: Utils.safeStringOrDefault(picture)
        anchors {
            margins: 6
        }
        groupAvatar: groupPicture
        avatarHeight: groupAvatar ? 40 : 44
    }

    states: [
        State {
            when: hoverHandler.containsMouse
            name: "hovering"
            PropertyChanges {
                target: bgBox
                color: CmnCfg.palette.sideBarHighlightColor
            }
        },
        State {
            when: parent.focus
            name: "selected"
            PropertyChanges {
                target: bgBox
                color: CmnCfg.palette.sideBarHighlightColor
            }
        }
    ]
}
