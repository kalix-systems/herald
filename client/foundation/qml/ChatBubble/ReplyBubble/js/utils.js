function jumpHandler(replyId, ownedConversation, convWindow) {
    const msgIndex = ownedConversation.indexById(replyId)

    if (msgIndex < 0)
        return

    const window = convWindow

    window.positionViewAtIndex(msgIndex, ListView.Center)
    window.highlightAnimation.target = window.itemAtIndex(
                msgIndex).highlightItem
    window.highlightAnimation.start()
}

function parseDocs(nameMetrics, modelData, fileSize) {
    const doc = JSON.parse(modelData.opDocAttachments)
    nameMetrics.text = doc.items[0].name
    fileSize.text = Utils.friendlyFileSize(doc.items[0].size)
    return doc.num_more
}

function parseMedia(modelData, imageClip) {
    const media = JSON.parse(modelData.opMediaAttachments)

    imageClip.imageSource = "file:" + media.items[0].path
    imageClip.count = (media.num_more === 0) ? media.items.length - 1 : media.num_more
                                               + media.items.length - 1
}
