import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0

// PAUL TODO : paramaterize the 12 dp margins
Item {
    // whether or not the AvatarIcon is square.
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
    property Component labelComponent
    // the initials to display in the icon
    property string initials
    readonly property real innerMargins: CmnCfg.smallMargin
    property color textColor: CmnCfg.palette.iconFill
    property real topTextMargin: 3
    property real bottomTextMargin: 4

    height: size
    width: size

    AvatarIcon {
        id: avatarIcon
        color: iconColor
        textColor: parent.textColor
        initials: parent.initials
        height: parent.size
        width: height
        pfpUrl: pfpPath
        anchors {
            verticalCenter: parent.verticalCenter
            left: parent.left
        }


    }

    Loader {
        id: labelContent
        anchors {
            leftMargin: CmnCfg.margin
            rightMargin: CmnCfg.margin
            topMargin: topTextMargin
            bottomMargin: bottomTextMargin

            left: avatarIcon.right
            right: parent.right
            top: avatarIcon.top
            bottom: avatarIcon.bottom
        }
        sourceComponent: labelComponent
    }

}
