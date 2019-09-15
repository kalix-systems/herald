export function deleteContact(index, contactsModel, messageModel, appRoot, heraldUtils) {
    const sameId = heraldUtils.compareByteArray(messageModel.conversationId, contactsModel.pairwiseConversationId(index));
    if (sameId) {
        appRoot.gsConversationId = undefined;
    }
    contactsModel.setStatus(index, 2 /* Deleted */);
}
export function renameContact(index, entryField, renameContactDialogue, contactsModel) {
    const trimmedText = entryField.text.trim();
    if (trimmedText === "") {
        return false;
    }
    const ret = contactsModel.setName(index, trimmedText);
    entryField.clear();
    renameContactDialogue.close();
    return ret;
}
