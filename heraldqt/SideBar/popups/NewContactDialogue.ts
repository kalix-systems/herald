export function insertContact(
  dialogue: Popup,
  entryArea: TextArea,
  contactsModel: Contacts
): void {
  const trimmedText = entryArea.text.trim();

  if (trimmedText.length === 0) {
    return;
  }

  contactsModel.add(trimmedText);
  entryArea.clear();
  dialogue.close();
}
