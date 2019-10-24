import QtQuick 2.13
import "../common" as Common
import Qt.labs.platform 1.1
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "Popups" as Popups

MouseArea {
    id: chatBubbleHitbox
    z: CmnCfg.underlayZ
    propagateComposedEvents: true
    hoverEnabled: true
    // KAAVYA: there has to be a better way to do this.
    width: parent.width + 50

    anchors {
        // Ternary is okay, types are enforced, cases are explicit.
        left: !outbound ? parent.left : undefined
        right: outbound ? parent.right : undefined
        bottom: parent.bottom
        top: parent.top
    }

    // PAUL : this should be a row
    Common.ButtonForm {
        id: messageOptionsButton
        visible: chatBubbleHitbox.containsMouse
        anchors {
            // Ternary is okay, types are enforced, cases are explicit.
            left: outbound ? parent.left : undefined
            right: !outbound ? parent.right : undefined
            margins: CmnCfg.margin
            verticalCenter: chatBubbleHitbox.verticalCenter
        }
        source: "qrc:/options-icon.svg"
        z: CmnCfg.overlayZ

        onClicked: messageOptionsMenu.open()
    }

    Popups.MessageOptionsPopup {
        id: messageOptionsMenu
    }

    Common.ButtonForm {
        id: replyButton
        visible: chatBubbleHitbox.containsMouse
        anchors {
            // Ternary is okay, types are enforced, cases are explicit.
            right: outbound ? messageOptionsButton.left : undefined
            left: !outbound ? messageOptionsButton.right : undefined
            margins: CmnCfg.margin
            verticalCenter: chatBubbleHitbox.verticalCenter
        }
        source: "qrc:/reply-icon.svg"
        z: 10

        onClicked: {
            chatTextArea.replyText = body
            chatTextArea.replyId = messageId
            chatTextArea.replyUid = author
            chatTextArea.replyName = contactsModel.nameById(author)
            builder.replyingTo = messageId
        }
    }
}
