import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0

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

    readonly property real topTextMargin: QmlCfg.units.dp(6)
    readonly property real bottomTextMargin: QmlCfg.units.dp(5)
    readonly property real innerMargins: QmlCfg.smallSpacer

    anchors {
        fill: parent
    }

    AvatarIcon {
        id: avatarIcon

        anchors {
            margins: innerMargins
            top: parent.top
            left: parent.left
        }

        color: iconColor
        height: QmlCfg.avatarSize
        width: height
        pfpUrl: pfpPath
    }

    Loader {
        id: labelContent
        anchors {
            top: parent.top
            bottom: parent.bottom
            right: parent.right
            left: avatarIcon.right
            margins: innerMargins
            topMargin: 0
        }
        sourceComponent: labelComponent
    }
}
