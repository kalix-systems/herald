function jumpHandler(replyId, ownedConversation, convWindow) {
  const msgIndex = ownedConversation.indexById(replyId);

  if (msgIndex < 0) return;

  const window = convWindow;

  window.positionViewAtIndex(msgIndex, ListView.Center);
  window.highlightAnimation.target = window.itemAtIndex(msgIndex).highlight;
  window.highlightAnimation.start();
}

function parseDocs(nameMetrics, modelData, fileSize, fileCount) {
  const doc = JSON.parse(modelData.opDocAttachments);
  nameMetrics.text = doc.first.name;
  fileSize.text = Utils.friendlyFileSize(doc.first.size);
  fileCount = doc.count - 1;
}

function parseMedia(modelData, imageClip) {
  const media = JSON.parse(modelData.opMediaAttachments);

  imageClip.imageSource = "file:" + media.first.path;
  imageClip.count = media.count - 1;
  imageClip.aspectRatio = media.first.width / media.first.height;
}
