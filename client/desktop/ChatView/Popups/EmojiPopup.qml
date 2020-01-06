import QtQuick 2.14
import LibHerald 1.0
import "../../EmojiKeyboard" as EK
import "../ChatTextArea/js/ChatTextAreaUtils.mjs" as JS

Loader {
    id: emoKeysPopup
    clip: true
    active: false
    onActiveChanged: emojiPickerModel.clearSearch()
    property bool isReactPopup: false
    sourceComponent: EK.EmojiPicker {
        id: emojiPicker
        //  z: chatPage.z + 2
        window: convWindow
        Component.onCompleted: {
            emojiPicker.send.connect(function (emoji) {
                if (!isReactPopup) {
                    JS.appendToTextArea(emoji, chatTextArea.chatText)
                } else {
                    ownedConversation.addReaction(index, emoji)
                }
            })
        }
        MouseArea {
            anchors.fill: parent
            z: parent.z - 1
        }
    }
}
