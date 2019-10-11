import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/utils.mjs" as Utils
import "../SideBar" as SideBar

Rectangle {
    property alias conversationItemAvatar: conversationItemAvatar
    property int boxColor
    property string boxTitle
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
        size: 45
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
