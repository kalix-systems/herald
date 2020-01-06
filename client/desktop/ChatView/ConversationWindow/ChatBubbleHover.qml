import QtQuick 2.13
import "../../common" as Common
import "qrc:/imports" as Imports
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../Popups" as Popups
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import QtQuick.Controls 2.3

MouseArea {
    id: chatBubbleHitbox
    property bool download: false
    propagateComposedEvents: true
    hoverEnabled: true
    anchors.fill: parent
    z: CmnCfg.overlayZ + 1
    onClicked: mouse.accepted = false
    onPressed: mouse.accepted = false
    onReleased: mouse.accepted = false
    onDoubleClicked: mouse.accepted = false
    onPositionChanged: mouse.accepted = false
    onPressAndHold: mouse.accepted = false

    Rectangle {
        id: buttonRect
        width: buttonRow.width
        height: buttonRow.height
        z: CmnCfg.overlayZ
        color: "transparent"
        visible: chatBubbleHitbox.containsMouse || parentBubble.hoverHighlight

        anchors {
            right: parent.right
            rightMargin: CmnCfg.microMargin
            top: parent.top
            topMargin: 6
        }

        Row {
            id: buttonRow
            anchors.right: parent.right
            topPadding: 0
            bottomPadding: 0
            spacing: CmnCfg.microMargin
            anchors.top: parent.top

            Imports.IconButton {
                id: replyButton
                anchors.margins: CmnCfg.defaultMargin
                fill: CmnCfg.palette.offBlack
                source: "qrc:/reply-icon.svg"
                z: CmnCfg.overlayZ
                icon.width: 14
                icon.height: 14
                padding: CmnCfg.microMargin

                // changing the opId transfers focus to the compose field
                onClicked: ownedConversation.builder.opId = msgId

                background: Rectangle {
                    border.color: CmnCfg.palette.offBlack
                    border.width: 1
                    color: CmnCfg.palette.white
                    radius: width * 0.3
                }
            }
            Imports.IconButton {
                id: reactButton
                anchors.margins: visible ? CmnCfg.defaultMargin : 0
                z: CmnCfg.overlayZ
                icon.width: visible ? 14 : 0
                icon.height: 14
                source: "qrc:/upside-down-emoji-icon.svg"
                fill: CmnCfg.palette.offBlack
                padding: CmnCfg.microMargin
                onClicked: {
                    reactPopup.active = true
                    emojiMenu.open()
                }
                background: Rectangle {
                    border.color: CmnCfg.palette.offBlack
                    border.width: 1
                    color: CmnCfg.palette.white
                    radius: width * 0.3
                }
            }
            Imports.IconButton {
                id: downloadButton
                visible: download
                anchors.margins: visible ? CmnCfg.defaultMargin : 0
                z: CmnCfg.overlayZ
                fill: CmnCfg.palette.offBlack
                icon.width: visible ? 14 : 0
                icon.height: 14
                padding: CmnCfg.microMargin
                source: "qrc:/download-icon.svg"
                onClicked: downloadFileChooser.open()
                background: Rectangle {
                    border.color: CmnCfg.palette.offBlack
                    border.width: 1
                    color: CmnCfg.palette.white
                    radius: width * 0.3
                }
            }
            Imports.IconButton {
                id: messageOptionsButton
                anchors.margins: CmnCfg.defaultMargin
                source: "qrc:/options-icon.svg"
                z: CmnCfg.overlayZ
                icon.width: 14
                icon.height: 14
                padding: CmnCfg.microMargin
                onClicked: messageOptionsMenu.open()
                fill: CmnCfg.palette.offBlack
                background: Rectangle {
                    border.color: CmnCfg.palette.offBlack
                    border.width: 1
                    color: CmnCfg.palette.white
                    radius: width * 0.3
                }
            }

            Popups.MessageOptionsPopup {
                id: messageOptionsMenu
            }
        }
    }
}
