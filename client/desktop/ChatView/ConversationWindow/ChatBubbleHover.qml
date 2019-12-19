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

    //    z: contentRoot.z + 1
    Row {
        spacing: CmnCfg.margin
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: CmnCfg.smallMargin
        anchors.verticalCenter: parent.verticalCenter

        Imports.ButtonForm {
            id: replyButton
            visible: chatBubbleHitbox.containsMouse
            anchors {
                margins: CmnCfg.margin
            }
            source: "qrc:/reply-icon.svg"
            z: CmnCfg.overlayZ

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
        FileDialog {
            id: downloadFileChooser
            selectFolder: true
            folder: StandardPaths.writableLocation(
                        StandardPaths.DesktopLocation)
            onAccepted: ownedConversation.saveAllAttachments(index, fileUrl)
            selectExisting: false
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
