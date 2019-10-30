export function insertContact(entryArea, contactsModel) {
    const trimmedText = entryArea.text.trim();
    if (trimmedText.length === 0) {
        return;
    }
    contactsModel.add(trimmedText);
    entryArea.clear();
}
