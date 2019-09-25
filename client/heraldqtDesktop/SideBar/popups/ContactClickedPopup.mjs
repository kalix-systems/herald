export function deleteContact(index, contactsModel, messageModel, appRoot, // TODO don't use this
heraldUtils) {
    const sameId = heraldUtils.compareByteArray(messageModel.conversationId, contactsModel.pairwiseConversationId(index));
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
export function changeProfilePicture(index, contactsModel, fileUrl, fileDialog) {
    const retCode = contactsModel.setProfilePicture(index, fileUrl);
    if (!retCode) {
        console.log("TODO: Native Error popup here...");
    }
    fileDialog.close();
}
