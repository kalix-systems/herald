export function insertContact(dialogue, entryArea, contactsModel) {
    const trimmedText = entryArea.text.trim();
    if (trimmedText.length === 0) {
        return;
    }
    contactsModel.add(trimmedText);
    entryArea.clear();
    dialogue.close();
}
