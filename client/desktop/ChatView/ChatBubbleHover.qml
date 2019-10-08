import QtQuick 2.13
import "../common" as Common
import Qt.labs.platform 1.1
import LibHerald 1.0

MouseArea {
    id: chatBubbleHitbox
    z: -1
    propagateComposedEvents: true
    hoverEnabled: true
    width: parent.width + 50

    onEntered: {
        messageOptionsButton.visible = !messageOptionsButton.visible
        replyButton.visible = !replyButton.visible
    }
    onExited: {
        messageOptionsButton.visible = !messageOptionsButton.visible
        replyButton.visible = !replyButton.visible
    }

    anchors {
        // Ternary is okay, types are enforced, cases are explicit.
        left: !outbound ? parent.left : undefined
        right: outbound ? parent.right : undefined
        bottom: parent.bottom
        top: parent.top
    }

    Common.ButtonForm {
        id: messageOptionsButton
        visible: false
        anchors {
            // Ternary is okay, types are enforced, cases are explicit.
            left: outbound ? parent.left : undefined
            right: !outbound ? parent.right : undefined
            margins: QmlCfg.margin
            verticalCenter: chatBubbleHitbox.verticalCenter
        }
        source: "qrc:/options-icon.svg"
        z: 10

        onClicked: {
            messageOptionsMenu.open()
        }

        Menu {
            id: messageOptionsMenu
            MenuItem {
                text: "Delete Message"
                onTriggered: ownedConversation.deleteMessage(index)
            }
            MenuItem {
                text: "More Info..."
            }
        }
    }

    Common.ButtonForm {
        id: replyButton
        visible: false
        anchors {
            // Ternary is okay, types are enforced, cases are explicit.
            right: outbound ? messageOptionsButton.left : undefined
            left: !outbound ? messageOptionsButton.right : undefined
            margins: QmlCfg.margin
            verticalCenter: chatBubbleHitbox.verticalCenter
        }
        source: "qrc:/reply-icon.svg"
        z: 10

        onClicked: {
            chatTextArea.replyText = body
            chatTextArea.replyId = messageId
            chatTextArea.replyUid = author
            chatTextArea.replyName = contactsModel.displayNameById(author)
            chatTextArea.state = "replystate"
        }
    }
}
