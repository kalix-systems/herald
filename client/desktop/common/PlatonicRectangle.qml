import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "../SideBar" as SideBar
import "qrc:/imports/Entity"

// Shared rectangle for displaying contact and conversation items in sidebar
// conversations lists, search results, and contact selection autocompletion
Rectangle {
    id: bgBox
    color: CmnCfg.palette.offBlack
    anchors.fill: parent
    property alias conversationItemAvatar: itemAvatar
    property string boxTitle
    property int boxColor
    property Component labelComponent
    property string picture
    property bool isGroupPicture: false
    property bool isMessageResult: false
    property int topTextMargin: CmnCfg.units.dp(6)
    property int bottomTextMargin: isMessageResult ? CmnCfg.largeMargin : CmnCfg.units.dp(
                                                         6)
    property alias label: conversationItemLabel.item

    Avatar {
        id: itemAvatar
        anchors {
            left: parent.left
            verticalCenter: parent.verticalCenter
            leftMargin: CmnCfg.smallMargin
        }
        color: CmnCfg.avatarColors[boxColor]
        initials: boxTitle[0].toUpperCase()
        pfpPath: Utils.safeStringOrDefault(picture)
        isGroup: isGroupPicture
    }

    Loader {
        id: conversationItemLabel
        anchors {
            leftMargin: CmnCfg.defaultMargin
            rightMargin: CmnCfg.defaultMargin
            topMargin: topTextMargin
            bottomMargin: bottomTextMargin
            left: itemAvatar.right
            right: parent.right
            top: parent.top
            bottom: parent.bottom
        }
        sourceComponent: bgBox.labelComponent
    }

    states: [
        State {
            when: hoverHandler.containsMouse
            name: "hovering"
            PropertyChanges {
                target: bgBox
                color: CmnCfg.palette.lightGrey
            }
        },
        State {
            when: parent.focus
            name: "selected"
            PropertyChanges {
                target: bgBox
                color: CmnCfg.palette.lightGrey
            }
        }
    ]
}
