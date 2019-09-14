export function insertContact(
  dialogue: Popup,
  entryArea: TextArea,
  contactsModel: Contacts,
  networkHandle: NetworkHandle
): void {
  const trimmedText = entryArea.text.trim();

  if (trimmedText.length === 0) {
    return;
  }

  const conversationId = contactsModel.add(trimmedText);
  networkHandle.sendAddRequest(trimmedText, conversationId);
  entryArea.clear();
  dialogue.close();
}
