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
    anchors.fill: bubbleActual
    z: 100
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
        // color: bubbleActual.authorColor
        z: CmnCfg.overlayZ
        color: "transparent"
        anchors {
            right: parent.right
            top: parent.top
            topMargin: CmnCfg.smallMargin
        }

        Row {
            id: buttonRow
            spacing: CmnCfg.margin
            rightPadding: CmnCfg.smallMargin

            Imports.ButtonForm {
                id: replyButton
                visible: chatBubbleHitbox.containsMouse
                anchors {
                    margins: CmnCfg.margin
                }
                source: "qrc:/reply-icon.svg"
                z: CmnCfg.overlayZ

                // changing the opId transfers focus to the compose field
                onClicked: ownedConversation.builder.opId = msgId
            }

            Popups.MessageOptionsPopup {
                id: messageOptionsMenu
            }

            //        ToolButton {
            //            text: qsTr("( ͡° ͜ʖ ͡°)")
            //            indicator: Item {
            //                width: 0
            //                height: 0
            //            }
            //            visible: chatBubbleHitbox.containsMouse
            //            display: AbstractButton.TextOnly
            //            anchors.margins: CmnCfg.margin
            //            spacing: 0
            //            padding: 0
            //        }
            Imports.ButtonForm {
                id: downloadButton
                visible: chatBubbleHitbox.containsMouse && download
                anchors {
                    margins: visible ? CmnCfg.margin : 0
                }
                z: CmnCfg.overlayZ
                icon.width: visible ? 22 : 0
                source: "qrc:/download-icon.svg"
                onClicked: downloadFileChooser.open()
            }

            Imports.ButtonForm {
                id: messageOptionsButton
                visible: chatBubbleHitbox.containsMouse

                anchors {
                    margins: CmnCfg.margin
                }
                source: "qrc:/options-icon.svg"
                z: CmnCfg.overlayZ
                onClicked: messageOptionsMenu.open()
            }
        }
    }
}
