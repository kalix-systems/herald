export function deleteContact(
  index: number,
  contactsModel: Contacts,
  messageModel: Messages,
  appRoot: GlobalState
): void {
  if (
    messageModel.conversationId === contactsModel.pairwiseConversationId(index)
  ) {
    appRoot.gsConversationId = undefined;
  }

  contactsModel.setStatus(index, ContactStatus.Deleted);
}
