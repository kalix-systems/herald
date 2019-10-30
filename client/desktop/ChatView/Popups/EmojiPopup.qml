import QtQuick 2.0
import "../../EmojiKeyboard" as EK
import "../Controls/js/ChatTextAreaUtils.mjs" as JS

Loader {
    id: emoKeysPopup
    clip: true
    active: false
    sourceComponent: EK.EmojiPicker {
        id: emojiPicker
        z: exit.z + 1
        window: convWindow
        Component.onCompleted: {
            emojiPicker.send.connect(function anon(emoji) {
                JS.appendToTextArea(emoji, chatTextArea.chatText)
            })
        }
    }
}
