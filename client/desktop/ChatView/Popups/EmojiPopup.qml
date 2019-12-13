import QtQuick 2.12
import LibHerald 1.0
import "../../EmojiKeyboard" as EK
import "../ChatTextArea/js/ChatTextAreaUtils.mjs" as JS

Loader {
    id: emoKeysPopup
    clip: true
    active: false
    sourceComponent: EK.EmojiPicker {
        id: emojiPicker
        z: exit.z + 2
        window: convWindow
        Component.onCompleted: {
            emojiPicker.send.connect(function (emoji) {
                JS.appendToTextArea(emoji, chatTextArea.chatText)
            })
        }
        MouseArea {
            anchors.fill: parent
            z: exit.z + 1
            propagateComposedEvents: false
        }
    }
}
