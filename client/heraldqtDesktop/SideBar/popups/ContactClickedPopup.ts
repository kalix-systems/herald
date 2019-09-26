export function deleteContact(
  index: number,
  contactsModel: Users,
  messageModel: Messages,
  heraldUtils: HeraldUtils
): void {
  const sameId = heraldUtils.compareByteArray(
    messageModel.conversationId,
    contactsModel.pairwiseConversationId(index)
  );

  contactsModel.setStatus(index, ContactStatus.Deleted);
}

export function renameContact(
  index: number,
  entryField: TextArea,
  renameContactDialogue: Popup,
  contactsModel: Users
): boolean {
  const trimmedText = entryField.text.trim();
  if (trimmedText === "") {
    return false;
  }

  const ret = contactsModel.setName(index, trimmedText);
  entryField.clear();
  renameContactDialogue.close();
  return ret;
}

export function changeProfilePicture(
  index: number,
  contactsModel: Users,
  fileUrl: string,
  fileDialog: Popup
): void {
  const retCode = contactsModel.setProfilePicture(index, fileUrl);

  if (!retCode) {
    console.log("TODO: Native Error popup here...");
  }

  fileDialog.close();
}
