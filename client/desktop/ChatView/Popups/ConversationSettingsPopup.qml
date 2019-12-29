import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports"
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/ChatBubble" as CB

Popup {
    id: conversationSettingsPopup
    property var convoMembers: parent.convoMembers

    height: chatView.height
    width: chatView.width
    anchors.centerIn: parent
    onClosed: messageInfoLoader.active = false

    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }


    Flickable {
        width: chatView.width
        height: chatView.height
        anchors.centerIn: parent
        contentWidth: width
        contentHeight: wrapperCol.height
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        Column {
            id: wrapperCol
            width: parent.width - CmnCfg.smallMargin * 2
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: CmnCfg.smallMargin
            topPadding: CmnCfg.smallMargin
            bottomPadding: CmnCfg.smallMargin
        }
    }
}
