export function deleteContact(pairwiseConversationId, index, contactsModel, messageModel, appRoot) {
    if (messageModel.conversationId === pairwiseConversationId) {
        appRoot.gsConversationId = undefined;
    }
    contactsModel.setStatus(index, 2 /* Deleted */);
    messageModel.clearConversationView();
}
