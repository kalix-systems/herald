import QtQuick 2.0

Keys {
    onReturnPressed: {
        if (event.modifiers & Qt.ShiftModifier) {
            chatText.text = chatText.text + "\n"
            chatText.cursorPosition = chatText.text.length
        } else {
            if (text.length <= 0) {
                return
            }
            if (text.trim().length === 0) {
                return
            }
            messageModel.insert_message(text)
            networkHandle.send_message(text, messageModel.conversationId)
            chatScrollBar.position = 1.0
            clear()
        }
    }
}
