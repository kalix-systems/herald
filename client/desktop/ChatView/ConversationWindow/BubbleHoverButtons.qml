import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12
import QtQuick 2.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/Entity"
import "." as CVUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "../../SideBar/js/ContactView.mjs" as CUtils
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import QtGraphicalEffects 1.0
import "../Popups" as Popups
import "qrc:/imports" as Imports

Rectangle {
    id: buttonRect
    width: buttonRow.width
    height: buttonRow.height
    color: "transparent"

    visible: parent.hitbox.containsMouse || parent.hoverHighlight
    anchors {
        right: parent.right
        rightMargin: CmnCfg.microMargin
        top: parent.top
        topMargin: 6
    }
    z: CmnCfg.overlayZ + 1

    Row {
        id: buttonRow
        anchors.right: parent.right
        topPadding: 0
        bottomPadding: 0
        spacing: CmnCfg.microMargin
        anchors.top: parent.top

        Imports.IconButton {
            id: replyButton
            tooltipText: "Reply to this message"
            anchors.margins: CmnCfg.defaultMargin
            fill: CmnCfg.palette.offBlack
            source: "qrc:/reply-icon-14.svg"
            z: bubbleActual.z + 1 //CmnCfg.overlayZ + 2
            icon.width: 14
            icon.height: 14
            padding: CmnCfg.microMargin

            // changing the opId transfers focus to the compose field
            onClicked: ownedConversation.builder.opId = msgId
            mouseArea.onEntered: {

                bubbleActual.hoverHighlight = true
                bubbleActual.expireInfo.visible = false
            }
            mouseArea.onExited: {
                if (!bubbleActual.hitbox.containsMouse)
                    bubbleActual.hoverHighlight = false
                if (isHead && !bubbleActual.hitbox.containsMouse)
                    bubbleActual.expireInfo.visible = true
            }

            background: Rectangle {
                border.color: CmnCfg.palette.offBlack
                border.width: 1
                color: replyButton.mouseArea.containsMouse ? CmnCfg.palette.lightGrey : CmnCfg.palette.white
            }
        }
        Imports.IconButton {
            id: reactButton
            tooltipText: "Add a react"
            anchors.margins: visible ? CmnCfg.defaultMargin : 0
            z: CmnCfg.overlayZ + 2
            icon.width: visible ? 14 : 0
            icon.height: 14
            source: "qrc:/emoticon-icon-14.svg"
            fill: CmnCfg.palette.offBlack
            padding: CmnCfg.microMargin
            onClicked: {
                emojiMenu.reactPopup.active = true
                emojiMenu.open()
            }
            background: Rectangle {
                border.color: CmnCfg.palette.offBlack
                border.width: 1
                color: reactButton.mouseArea.containsMouse ? CmnCfg.palette.lightGrey : CmnCfg.palette.white
            }
            mouseArea.onEntered: {

                bubbleActual.hoverHighlight = true
                bubbleActual.expireInfo.visible = false
            }
            mouseArea.onExited: {
                if (!bubbleActual.hitbox.containsMouse)
                    bubbleActual.hoverHighlight = false
                if (isHead && !bubbleActual.hitbox.containsMouse)
                    bubbleActual.expireInfo.visible = true
            }
        }
        Imports.IconButton {
            id: downloadButton
            tooltipText: "Download all attachments"
            visible: bubbleActual.hitbox.download
            anchors.margins: visible ? CmnCfg.defaultMargin : 0
            z: CmnCfg.overlayZ + 2
            fill: CmnCfg.palette.offBlack
            icon.width: visible ? 14 : 0
            icon.height: 14
            padding: CmnCfg.microMargin
            source: "qrc:/download-icon-14.svg"
            onClicked: downloadFileChooser.open()
            background: Rectangle {
                border.color: CmnCfg.palette.offBlack
                border.width: 1
                color: downloadButton.mouseArea.containsMouse ? CmnCfg.palette.lightGrey : CmnCfg.palette.white
            }
            mouseArea.onEntered: {
                bubbleActual.hoverHighlight = true
                bubbleActual.expireInfo.visible = false
            }
            mouseArea.onExited: {
                if (!bubbleActual.hitbox.containsMouse)
                    bubbleActual.hoverHighlight = false
                if (isHead && !bubbleActual.hitbox.containsMouse)
                    bubbleActual.expireInfo.visible = true
            }
        }
        Imports.IconButton {
            id: messageOptionsButton
            tooltipText: "More options"
            anchors.margins: CmnCfg.defaultMargin
            source: "qrc:/options-icon-14.svg"
            z: CmnCfg.overlayZ + 2
            icon.width: 14
            icon.height: 14
            padding: CmnCfg.microMargin
            onClicked: messageOptionsMenu.open()
            fill: CmnCfg.palette.offBlack
            background: Rectangle {
                border.color: CmnCfg.palette.offBlack
                border.width: 1
                color: messageOptionsButton.mouseArea.containsMouse ? CmnCfg.palette.lightGrey : CmnCfg.palette.white
            }

            mouseArea.onEntered: {
                bubbleActual.hoverHighlight = true
                bubbleActual.expireInfo.visible = false
            }

            mouseArea.onExited: {
                if (!bubbleActual.hitbox.containsMouse)
                    bubbleActual.hoverHighlight = false
                if (isHead && !bubbleActual.hitbox.containsMouse)
                    bubbleActual.expireInfo.visible = true
            }
        }

        Popups.MessageOptionsPopup {
            id: messageOptionsMenu
        }
    }
}
