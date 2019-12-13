import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "./ReplyBubble"
import "../js/utils.mjs" as Utils

Rectangle {
    id: bubbleRoot

    property real defaultWidth
    property bool elided: body.length !== messageModelData.fullBody.length
    property bool expanded: false
    property bool outbound: parent.outbound
    property Item convContainer
    property var messageModelData

    property alias highlightItem: bubbleHighlight

    readonly property color bubbleColor: CmnCfg.palette.lightGrey
    readonly property bool highlight: messageModelData.matchStatus === 2

    readonly property string body: messageModelData.body
    readonly property string authorId: messageModelData.author
    readonly property string authorName: Herald.users.nameById(authorId)

    readonly property string medAttachments: messageModelData.mediaAttachments
    readonly property string documentAttachments: messageModelData.docAttachments
    readonly property bool imageAttach: medAttachments.length !== 0
    readonly property bool docAttach: documentAttachments.length !== 0

    readonly property var replyId: messageModelData.opMsgId
    readonly property bool reply: messageModelData.replyType > 0

    readonly property real maxWidth: imageAttach ? 300 : Math.min(defaultWidth,
                                                                  600)
    readonly property string friendlyTimestamp: Utils.friendlyTimestamp(
                                                    messageModelData.insertionTime)
    readonly property string receiptImage: Utils.receiptCodeSwitch(
                                               messageModelData.receiptStatus)
    readonly property color authorColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                                 authorId)]

    color: bubbleColor
    width: contentRoot.width
    height: contentRoot.height

    Highlight {
        id: bubbleHighlight
    }

    Rectangle {
        id: verticalAccent
        anchors.right: !outbound ? contentRoot.left : undefined
        anchors.left: outbound ? contentRoot.right : undefined
        height: contentRoot.height
        width: CmnCfg.smallMargin / 4
        color: authorColor
    }

    Column {
        id: contentRoot

        // Text edit alias
        property alias messageBody: messageBody
        /// User name label alias
        property alias messageLabel: uname

        // all messages are un-expanded on completion
        Component.onCompleted: bubbleRoot.expanded = false

        spacing: 0
        //image component
        Component {
            id: image
            AttachmentContent {}
        }

        // document component
        Component {
            id: doc
            FileAttachmentContent {}
        }

        //reply bubble loader
        Loader {
            sourceComponent: {
                if (!reply)
                    return undefined

                const hasDoc = messageModelData.opDocAttachments.length > 0
                const hasMedia = messageModelData.opMediaAttachments.length > 0

                if (hasDoc && hasMedia) {
                    return replyHybridContent
                } else if (hasDoc) {
                    return replyDocContent
                } else if (hasMedia) {
                    return replyMediaContent
                } else {
                    return replyContent
                }
            }

            // reply bubble if there is doc file content
            Component {
                id: replyHybridContent
                ReplyHybrid {
                    maxWidth: contentRoot.maxWidth
                    replyId: contentRoot.replyId
                    modelData: contentRoot.messageModelData
                }
            }

            // reply bubble if there is doc file content
            Component {
                id: replyDocContent
                ReplyDoc {
                    maxWidth: contentRoot.maxWidth
                    replyId: contentRoot.replyId
                    modelData: contentRoot.messageModelData
                }
            }

            // reply media bubble if there is media file content
            Component {
                id: replyMediaContent
                ReplyImage {}
            }

            // reply bubble if there is no doc file content
            Component {
                id: replyContent
                ReplyText {
                    // maxWidth: bubbleRoot.maxWidth
                    replyId: bubbleRoot.replyId
                }
            }
        }

        //author name
        ChatLabel {
            id: uname
            senderName: authorName
            senderColor: authorColor
        }

        //media and file column loader
        Column {
            Loader {
                id: imageLoader
                sourceComponent: imageAttach ? image : undefined
            }

            Loader {
                id: fileLoader
                sourceComponent: docAttach ? doc : undefined
            }
        }

        //message body
        StandardTextEdit {
            id: messageBody
            maximumWidth: bubbleRoot.imageAttach ? 300 : Math.min(
                                                       bubbleRoot.maxWidth, 600)
        }

        ElideHandler {}

        StandardStamps {}
    }
}
