



function enterHandler(target) {

    if (event.modifiers & Qt.ShiftModifier) {
                target.text = target.text + "\n"
                target.cursorPosition = target.text.length
            } else {
                if (target.text.length <= 0) {
                    return
                }
                if (target.text.trim().length === 0) {
                    return
                }
                var result = networkHandle.send_message(
                            target.text, messageModel.conversationId)
                messageModel.insert_message(target.text, result)
                chatScrollBar.position = 1.0
                target.clear()
            }

}
