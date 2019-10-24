import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../../foundation/js/utils.mjs" as Utils
import "../SideBar" as SideBar
import "qrc:/imports/Avatar"

// Shared rectangle for displaying contact and conversation items in sidebar
Rectangle {
    property alias conversationItemAvatar: conversationItemAvatar
    property int boxColor
    // title of the contact/convo
    property string boxTitle
    // is true if it's a contact, false if it's a conversation
    property bool isContact
    id: bgBox
    color: CmnCfg.palette.paneColor
    anchors.fill: parent


    AvatarMain {
        id: conversationItemAvatar
        iconColor: CmnCfg.avatarColors[Utils.unwrapOr(boxColor, 0)]
        textColor: CmnCfg.palette.iconFill
        size: 42
        initials: boxTitle[0].toUpperCase()
        pfpPath: Utils.safeStringOrDefault(picture)
        anchors {
            margins: 6
        }


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
