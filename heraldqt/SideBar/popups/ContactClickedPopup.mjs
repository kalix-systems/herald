export function deleteContact(index, contactsModel, messageModel, appRoot) {
    if (messageModel.conversationId === contactsModel.pairwiseConversationId(index)) {
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
