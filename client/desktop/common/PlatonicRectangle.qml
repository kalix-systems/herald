import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/js/utils.mjs" as Utils
import "../SideBar" as SideBar

// Shared rectangle for displaying contact and conversation items in sidebar
Rectangle {
    property alias conversationItemAvatar: conversationItemAvatar
    // color of the contact/convo
    property int boxColor
    // title of the contact/convo
    property string boxTitle
    // is true if it's a contact, false if it's a conversation
    property bool isContact
    id: bgBox
    color: QmlCfg.palette.mainColor
    anchors.fill: parent

    Common.Divider {
        color: QmlCfg.palette.secondaryColor
        anchor: parent.bottom
        // PAUL: convert to device independent size this is magic.
        height: 2
    }

    Common.Avatar {
        id: conversationItemAvatar
        // PAUL: convert to device independent size this is magic.
        size: QmlCfg.avatarSize
        labeled: isContact
        labelGap: QmlCfg.smallMargin
        avatarLabel: boxTitle
        colorHash: Utils.unwrapOr(boxColor, 0)
        pfpUrl: Utils.safeStringOrDefault(picture)
        secondaryText: isContact ? "@" + userId : ""
    }

    states: [
        State {
            when: hoverHandler.containsMouse
            name: "hovering"
            PropertyChanges {
                target: bgBox
                color: QmlCfg.palette.secondaryColor
            }
        },
        State {
            when: parent.focus
            name: "selected"
            PropertyChanges {
                target: bgBox
                color: QmlCfg.palette.tertiaryColor
            }
        }
    ]
}
