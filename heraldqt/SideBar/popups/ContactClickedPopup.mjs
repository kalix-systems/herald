export function deleteContact(index, contactsModel, messageModel, appRoot) {
    if (messageModel.conversationId === contactsModel.pairwiseConversationId(index)) {
        appRoot.gsConversationId = undefined;
    }
    contactsModel.setStatus(index, 2 /* Deleted */);
}
