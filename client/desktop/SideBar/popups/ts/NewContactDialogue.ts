export function insertContact(entryArea: TextArea, contactsModel: Users): void {
  const trimmedText = entryArea.text.trim();
  if (trimmedText.length === 0) {
    return;
  }
  contactsModel.add(trimmedText);
  entryArea.clear();
}
