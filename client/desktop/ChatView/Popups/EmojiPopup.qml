import QtQuick 2.12
import LibHerald 1.0
import "../../EmojiKeyboard" as EK
import "../ChatTextArea/js/ChatTextAreaUtils.mjs" as JS

Loader {
    id: emoKeysPopup
    clip: true
    active: false
    property bool isReactPopup: false
    property int currentIndex
    sourceComponent: EK.EmojiPicker {
        id: emojiPicker
        z: exit.z + 2
        window: convWindow
        Component.onCompleted: {
            emojiPicker.send.connect(function (emoji) {
                if (!isReactPopup) {
                    JS.appendToTextArea(emoji, chatTextArea.chatText)
                } else {
                    print(currentIndex, emoji)
                    ownedConversation.addReaction(currentIndex, emoji)
                }
            })
        }
        MouseArea {
            anchors.fill: parent
            z: exit.z + 1
            propagateComposedEvents: false
        }
    }
}
