import QtQuick 2.13
import "../common" as Common
import LibHerald 1.0

MouseArea {
    propagateComposedEvents: true
    id: chatBubbleHitbox
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
            chatTextArea.state = "replystate"
            chatTextArea.replyId = messageId
            chatTextArea.replyText = body
        }
    }
}
