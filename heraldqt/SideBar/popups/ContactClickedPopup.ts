export function deleteContact(
  pairwiseConversationId: ConversationID,
  index: number,
  contactsModel: Contacts,
  messageModel: Messages,
  appRoot: GlobalState
): void {
  if (messageModel.conversationId === pairwiseConversationId) {
    appRoot.gsConversationId = undefined;
  }

  contactsModel.setStatus(index, ContactStatus.Deleted);
  messageModel.clearConversationView();
}
