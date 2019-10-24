import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0

// PAUL TODO : paramaterize the 12 dp margins
Item {
    // weather or not the AvatarIcon is square.
    property bool groupAvatar: false
    // path to the profile picture, or the empty string
    property string pfpPath
    // the color with which to fill the icon if there is not profile picture
    property color iconColor
    property real size: CmnCfg.avatarSize
    // the label, must be one of the sibling components in this directory with the Label suffix
    // for example
    //```
    //  AvatarMain {
    //      labelComponent: ConversationLabel
    //           {
    //              color : CmnCfg.avatarColors[colorCode]
    //              ...
    //            }
    //        }
    //```
    property Component labelComponent: ConversationLabel {}
    // the initials to display in the icon
    property string initials
    readonly property real innerMargins: CmnCfg.smallMargin
    property real topTextMargin: CmnCfg.units.dp(4)
    property real bottomTextMargin: CmnCfg.units.dp(4)
    anchors.fill: parent

    AvatarIcon {
        id: avatarIcon
        color: iconColor
        initials: parent.initials
        height: size
        width: height
        pfpUrl: pfpPath
        anchors {
            verticalCenter: parent.verticalCenter
            left: parent.left
        }
    }

    Item {
        anchors {
            top: avatarIcon.top
            bottom: avatarIcon.bottom
            left: avatarIcon.right
            right: parent.right
            topMargin: topTextMargin
            bottomMargin: bottomTextMargin
            leftMargin: CmnCfg.units.dp(12)
            rightMargin: CmnCfg.units.dp(12)
        }
        Loader {
            id: labelContent
            anchors.fill: parent
            sourceComponent: labelComponent
        }
    }
}
