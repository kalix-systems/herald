import QtQuick 2.13
import "../../common" as Common
import "qrc:/imports" as Imports
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../Popups" as Popups

MouseArea {
    id: chatBubbleHitbox
    z: CmnCfg.underlayZ
    propagateComposedEvents: true
    hoverEnabled: true
    width: parent.width + 50

    anchors {
        left: !outbound ? parent.left : undefined
        right: outbound ? parent.right : undefined
        bottom: parent.bottom
        top: parent.top
    }

    Imports.ButtonForm {
        id: messageOptionsButton
        visible: chatBubbleHitbox.containsMouse

        anchors {
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

    Imports.ButtonForm {
        id: replyButton
        visible: chatBubbleHitbox.containsMouse
        anchors {
            right: outbound ? messageOptionsButton.left : undefined
            left: !outbound ? messageOptionsButton.right : undefined
            margins: CmnCfg.margin
            verticalCenter: chatBubbleHitbox.verticalCenter
        }
        source: "qrc:/reply-icon.svg"
        z: CmnCfg.overlayZ

        onClicked: ownedConversation.builderOpMsgId = msgId
    }
}
