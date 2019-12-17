import QtQuick 2.13
import "../../common" as Common
import "qrc:/imports" as Imports
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../Popups" as Popups
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3

MouseArea {
    id: chatBubbleHitbox
    z: CmnCfg.underlayZ
    property bool download: false
    propagateComposedEvents: true
    hoverEnabled: true
    width: download ? parent.width + 110 : parent.width + 80

    anchors {
        left: !outbound ? parent.left : undefined
        right: outbound ? parent.right : undefined
        bottom: parent.bottom
        top: parent.top
    }
    Row {
        spacing: CmnCfg.margin
        anchors.left: outbound ? parent.left : undefined
        anchors.right: !outbound ? parent.right : undefined
        anchors.verticalCenter: parent.verticalCenter
        layoutDirection: outbound ? Qt.LeftToRight : Qt.RightToLeft
        Imports.ButtonForm {
            id: messageOptionsButton
            visible: chatBubbleHitbox.containsMouse

            anchors {
                margins: CmnCfg.margin
                verticalCenter: parent.verticalCenter
            }
            source: "qrc:/options-icon.svg"
            z: CmnCfg.overlayZ
            onClicked: messageOptionsMenu.open()
        }

        Popups.MessageOptionsPopup {
            id: messageOptionsMenu
        }

        Imports.ButtonForm {
            id: downloadButton
            visible: chatBubbleHitbox.containsMouse && download
            anchors {
                verticalCenter: parent.verticalCenter
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
            id: replyButton
            visible: chatBubbleHitbox.containsMouse
            anchors {
                margins: CmnCfg.margin
                verticalCenter: parent.verticalCenter
            }
            source: "qrc:/reply-icon.svg"
            z: CmnCfg.overlayZ

            onClicked: ownedConversation.builder.opId = msgId
        }
    }
}
