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
        color: bubbleActual.aux ? "transparent" : "white"
        visible: chatBubbleHitbox.containsMouse || parentBubble.hoverHighlight

        Rectangle {
            anchors.fill: parent
            color: bubbleActual.aux ? "transparent" : bubbleActual.highlightItem.color
            opacity: 0.2
        }

        anchors {
            right: parent.right
            rightMargin: CmnCfg.microMargin
            top: parent.top
            topMargin: CmnCfg.smallMargin
        }

        Grid {
            id: buttonRow
            anchors.right: parent.right
            property bool colMode: reply && !isHead
            columns: {
                if (colMode) {
                    return 1
                } else {
                    return download ? 4 : 3
                }
            }
            flow: {
                if (colMode)
                    Grid.TopToBottom
                else
                    Grid.LeftToRight
            }

            width: if (colMode) {
                       return 22
                   }

            topPadding: 0
            bottomPadding: 0
            spacing: CmnCfg.smallMargin

            Imports.IconButton {
                id: replyButton
                anchors.margins: CmnCfg.defaultMargin
                source: "qrc:/reply-icon.svg"
                z: CmnCfg.overlayZ
                // changing the opId transfers focus to the compose field
                onClicked: ownedConversation.builder.opId = msgId
            }
            Imports.IconButton {
                id: reactButton
                anchors.margins: visible ? CmnCfg.defaultMargin : 0
                z: CmnCfg.overlayZ
                icon.width: visible ? 24 : 0
                source: "qrc:/upside-down-emoji-icon.svg"
                onClicked: {
                    reactPopup.active = true
                    emojiMenu.open()
                }
            }
            Imports.IconButton {
                id: downloadButton
                visible: download
                anchors.margins: visible ? CmnCfg.defaultMargin : 0
                z: CmnCfg.overlayZ
                icon.width: visible ? 22 : 0
                source: "qrc:/download-icon.svg"
                onClicked: downloadFileChooser.open()
            }
            Imports.IconButton {
                id: messageOptionsButton
                anchors.margins: CmnCfg.defaultMargin
                source: "qrc:/options-icon.svg"
                z: CmnCfg.overlayZ
                onClicked: messageOptionsMenu.open()
            }

            Popups.MessageOptionsPopup {
                id: messageOptionsMenu
            }
        }
    }
}
