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
    // the label, must be one of the sibling components in this directory with the Label suffix
    // for example
    //```
    //  AvatarMain {
    //      labelComponent: ConversationLabel
    //           {
    //              color : QmlCfg.avatarColors[colorCode]
    //              ...
    //            }
    //        }
    //```
    property Component labelComponent: ConversationLabel {}
    // the initials to display in the icon
    property string initials
    readonly property real topTextMargin: QmlCfg.units.dp(6)
    readonly property real bottomTextMargin: QmlCfg.units.dp(5)
    readonly property real innerMargins: QmlCfg.smallSpacer

    anchors.fill: parent

    AvatarIcon {
        id: avatarIcon
        color: iconColor
        initials: parent.initials
        height: QmlCfg.avatarSize
        width: height
        pfpUrl: pfpPath
        anchors {
            verticalCenter: parent.verticalCenter
            left: parent.left
            margins: QmlCfg.units.dp(12)
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
            leftMargin: QmlCfg.units.dp(12)
            rightMargin: QmlCfg.units.dp(12)
        }
        Loader {
            id: labelContent
            anchors.fill: parent
            sourceComponent: labelComponent
        }
    }
}
