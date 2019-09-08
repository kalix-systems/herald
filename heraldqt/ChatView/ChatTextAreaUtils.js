
/*
* a memory tuple class
**/
class ChatViewMemory {
    constructor() {
        this.scrollMemory = Number(1.0)
        this.textMemory = String("")
    }
}

/*
* global object to keep track of  chat area memory
**/
var textAreaMemory = {
    currentConversationid: "",
    invalid: new ChatViewMemory()
};

/*
* Gets chatview memory with the current conversationID
* on falsey conversation id returns
*
*
*
**/
function getTextAreaMemory(conversationId) {
    if (!conversationID) {
        return textAreaMemory.invalid
    }
}


function enterHandler(event, target) {

    if (event.modifiers & Qt.ShiftModifier) {
        target.text = target.text + "\n"
        target.cursorPosition = target.text.length
        return
    }

    if (target.text.trim().length <= 0) {
        return
    }

    // clear before positional reset
    let text = target.text
    target.clear()

    var result = networkHandle.send_message(text, messageModel.conversationId)
    messageModel.insert_message(text, result)
}

