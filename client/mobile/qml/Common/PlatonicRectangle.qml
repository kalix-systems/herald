import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import "../Common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/Entity"

// Shared rectangle for displaying contact and conversation items in sidebar
// conversations lists, search results, and contact selection autocompletion
Rectangle {
    id: bgBox
    anchors.fill: parent
    property alias conversationItemAvatar: itemAvatar
    property string boxTitle
    property int boxColor
    property Component labelComponent
    property string picture
    property bool isGroupPicture: false
    property bool isMessageResult: false
    property int topTextMargin: CmnCfg.defaultMargin
    property int bottomTextMargin: CmnCfg.defaultMargin

    Avatar {
        id: itemAvatar
        anchors {
            left: parent.left
            verticalCenter: parent.verticalCenter
            leftMargin: CmnCfg.smallMargin
        }
        color: CmnCfg.avatarColors[boxColor]
        initials: Utils.initialize(boxTitle)
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
            top: parent.top
            bottom: parent.bottom
            left: itemAvatar.right
            right: parent.right
        }
        sourceComponent: bgBox.labelComponent
    }
}
