export function isOnscreen(
	ownedConversation: Messages, 
	chatListView: Repeater,
	chatPane: Page,
	forward: boolean
	): boolean {

	if (!forward) {
		const x = chatListView.itemAt(ownedConversation.peekPrevSearchMatch()).x;
		const y = chatListView.itemAt(ownedConversation.peekPrevSearchMatch()).y;
		const yPos = chatPane.mapFromItem(chatListView, x, y).y;
		const pageHeight = chatPane.height - 50;

		if (0 < yPos && yPos < pageHeight) {
			return true;
		}
		else {
			return false;
			
			}
	}

	else {
		const x = chatListView.itemAt(ownedConversation.peekNextSearchMatch()).x;
		const y = chatListView.itemAt(ownedConversation.peekNextSearchMatch()).y;
		const yPos = chatPane.mapFromItem(chatListView, x, y).y;
		const pageHeight = chatPane.height - 50;

		if (0 < yPos && yPos < pageHeight) {
			return true;
		}
		else {
			return false;
		}

	}

}

export function jumpHandler(
	ownedConversation: Messages,
	chatListView: Repeater,
	chatPane: Page,
	conversationWindow: ConversationWindow,
	forward: boolean
	): void {

	const toJump = !isOnscreen(ownedConversation, chatListView, chatPane, forward);

	const convoMiddle = conversationWindow.height / 2

	if (forward) {
		if (toJump) {
			conversationWindow.contentY = chatListView.itemAt(ownedConversation.nextSearchMatch()).y - convoMiddle;
		}
		else {
			ownedConversation.nextSearchMatch()
		}
		return;
	}

	else {
		if (toJump) {
			conversationWindow.contentY = chatListView.itemAt(ownedConversation.prevSearchMatch()).y - convoMiddle;
		}
		else {
			ownedConversation.prevSearchMatch()

		}

		return;
	}
}