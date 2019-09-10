// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC

//TS: This whole file.

/*
 * a memory tuple class
 **/
export class ChatViewMemory {
  scrollMemory: number;
  textMemory: string;

  constructor() {
    this.scrollMemory = Number(1.0);
    this.textMemory = String("");
  }
}

/*
 * global object to keep track of  chat area memory
 **/
export const textAreaMemory = {
  currentConversationid: "",
  invalid: new ChatViewMemory()
};

/*
 * Gets chatview memory with the current conversationID
 * on falsey conversation id returns
 **/
export function getTextAreaMemory(
  conversationID: number
): ChatViewMemory | undefined {
  if (!!!conversationID) {
    return textAreaMemory.invalid;
  }
}

export function enterKeyHandler(
  event: QKeyEvent,
  target: TextArea,
  networkHandle: NetworkHandle,
  messageModel: Messages
): void {
  if (event.modifiers & Qt.ShiftModifier) {
    target.text = target.text + "\n";
    target.cursorPosition = target.text.length;
    return;
  }

  if (target.text.trim().length <= 0) {
    return;
  }

  // clear before positional reset
  const text = target.text;
  target.clear();

  const result = networkHandle.sendMessage(text, messageModel.conversationId);
  messageModel.insertMessage(text, result);
}
