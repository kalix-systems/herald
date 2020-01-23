import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "./ReplyBubble"
import "../js/utils.mjs" as Utils
import "../Entity"

//This is a default bubble that e.g. displays avatar, has default padding regardless of
//isHead and isTail. It is used in the view for more info for a message
Rectangle {
    id: bubbleRoot

    property real defaultWidth
    property bool isNull: messageModelData === null

    // elided: describes whether or not the body of this message has been shortened
    // by helper functions defined in rust.
    property bool elided: isNull ? false : body.length !== messageModelData.fullBody.length
    // expanded: true if the message was elided and is now showing the full body after the user presses
    // the read more button
    property bool expanded: false
    // outbound: true if the user is the author of this message
    property bool outbound: isNull ? false : messageModelData.author === Herald.config.configId

    property Item convContainer

    property var messageModelData

    // highlightItem: an exported component which is simply the rectangle that colors
    // the bubble on hover.
    property alias highlightItem: bubbleHighlight

    readonly property color bubbleColor: CmnCfg.palette.lightGrey
    // highlight: whether or not the current message has been found in a user search
    // query. referring to the fact that part of the message is in a highlight span.
    readonly property bool highlight: isNull ? false : messageModelData.matchStatus === 2

    // body: the displayed message body
    readonly property string body: isNull ? "" : messageModelData.body
    // authorId: the userId of the author
    readonly property string authorId: isNull ? "" : messageModelData.author
    // authorName: the display name of the author
    readonly property string authorName: isNull ? "" : messageModelData.authorName
    // medAttachments: a JSON serialized string listing all media attachments. includes
    // information about their dimensions. this is only the first six.
    readonly property string medAttachments: isNull ? "" : messageModelData.mediaAttachments
    // fullMedAttachments: See medAttachments
    readonly property string fullMedAttachments: isNull ? "" : messageModelData.fullMediaAttachments

    readonly property string documentAttachments: isNull ? "" : messageModelData.docAttachments
    // imageAttach: whether or not this message has an image attached to it.
    readonly property bool imageAttach: isNull ? false : medAttachments.length !== 0
    // docAttach: whether or not this message has any non-previewable attachments.
    readonly property bool docAttach: isNull ? false : documentAttachments.length !== 0

    // replyId: The message ID QByteArray corresponding to the message that this message is replying to, if any.
    readonly property var replyId: isNull ? undefined : messageModelData.opMsgId
    // reply: whether or this message is a reply to a pre existing message
    readonly property bool reply: isNull ? false : messageModelData.replyType > 0

    // isHead: true if this message is the first of a logical flurry of messages from the same user.
    // a logical flurry is a group of messages that have not been interrupted by a message from another user,
    // with less than five minutes (CHECK ME) inbetween each message.
    readonly property bool isHead: isNull ? false : messageModelData.isHead
    // isTail: true if this message is the last message in a logical flurry. see isHead.
    readonly property bool isTail: isNull ? false : messageModelData.isTail
    // hasReactions: whether or not this messages has an emoji reaction associated with it.
    readonly property bool hasReactions: isNull ? false : messageModelData.reactions.length > 0

    readonly property real maxWidth: defaultWidth * 0.75
    // friendlyTimestamp: the user friendly timestamp corresponding to when this message was received
    property string friendlyTimestamp: isNull ? "" : Utils.friendlyTimestamp(
                                                    messageModelData.insertionTime)
    // timerIcon: the rcc path corresponding to the hourglass icon used to indicate how much time is left before this message
    // self destructs.
    property string timerIcon: isNull ? "" : messageModelData.expirationTime
                                        !== undefined ? Utils.timerIcon(
                                                            messageModelData.expirationTime,
                                                            messageModelData.insertionTime) : ""
    // receiptImage: the rcc path corresponding to the check mark icon indicating whether or not the receipient of this message
    // has seen it or not.
    readonly property string receiptImage: isNull ? "" : outbound ? Utils.receiptCodeSwitch(
                                                                        messageModelData.receiptStatus) : ""
    // authorColor: the QColor corresponding to the user set hue used to color the flair of the message.
    readonly property color authorColor: isNull ? "white" : CmnCfg.avatarColors[messageModelData.authorColor]

    // pfpUrl: the file url corresponding to the authors profile picture.
    readonly property string pfpUrl: isNull ? "" : messageModelData.authorProfilePicture

    // hoverHighlight: whether or not this item is currently hovered, showing a color overlay, i.e. `highlightItem`
    property bool hoverHighlight: false

    // moreInfo: (CHECK ME)
    property bool moreInfo: true

    height: contentRoot.height
    width: defaultWidth

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {
            friendlyTimestamp = Utils.friendlyTimestamp(
                        messageModelData.insertionTime)
            timerIcon = (messageModelData.expirationTime
                         !== undefined) ? (Utils.timerIcon(
                                               messageModelData.expirationTime,
                                               messageModelData.insertionTime)) : ""

            expireInfo.expireTime = (messageModelData.expirationTime
                                     !== undefined) ? (Utils.expireTimeShort(
                                                           messageModelData.expirationTime,
                                                           messageModelData.insertionTime)) : ""
        }
    }
    color: CmnCfg.palette.white

    Rectangle {
        anchors.top: parent.top
        width: parent.width
        height: 1
        color: CmnCfg.palette.medGrey
    }

    Rectangle {
        anchors.bottom: parent.bottom
        width: parent.width

        height: 1
        color: CmnCfg.palette.medGrey
    }

    Highlight {
        id: bubbleHighlight
        z: bubbleRoot.z + 1
    }

    Avatar {
        id: avatar
        color: authorColor
        initials: isNull ? "" : authorName[0].toUpperCase()
        size: 36

        anchors {
            left: parent.left
            top: parent.top
            margins: CmnCfg.smallMargin
        }

        z: contentRoot.z + 1
        pfpPath: pfpUrl
    }

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.accentBarWidth
        color: authorColor
        anchors.left: avatar.right
        anchors.leftMargin: CmnCfg.smallMargin
    }

    BubbleExpireInfo {
        id: expireInfo
    }

    Button {
        id: receipt
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: CmnCfg.smallMargin

        icon.source: receiptImage
        icon.height: CmnCfg.units.dp(14)
        icon.width: CmnCfg.units.dp(14)
        icon.color: CmnCfg.palette.darkGrey
        padding: 0
        background: Item {}
    }

    Column {
        z: highlight.z + 1
        id: contentRoot
        anchors.left: accent.right
        // all messages are un-expanded on completion
        Component.onCompleted: bubbleRoot.expanded = false

        spacing: CmnCfg.smallMargin
        topPadding: CmnCfg.smallMargin
        leftPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.defaultMargin

        BubbleLabel {
            id: authorLabel
            timestamp: friendlyTimestamp
        }

        //reply bubble loader
        Loader {
            sourceComponent: {
                if (!reply)
                    return undefined

                if (messageModelData.replyType === 1) {
                    return replyDanglingContent
                }

                if (messageModelData.opAuxData.length > 0) {
                    return replyAux
                }

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
                    mouseEnabled: false
                }
            }

            // reply bubble if there is doc file content
            Component {
                id: replyDanglingContent
                ReplyDangling {}
            }

            // reply bubble if there is doc file content
            Component {
                id: replyDocContent
                ReplyDoc {
                    mouseEnabled: false
                }
            }

            // reply media bubble if there is media file content
            Component {
                id: replyMediaContent
                ReplyImage {
                    mouseEnabled: false
                }
            }

            // reply bubble if there is no doc file content
            Component {
                id: replyContent
                ReplyText {
                    mouseEnabled: false
                }
            }

            // reply bubble if it is an aux message
            Component {
                id: replyAux
                ReplyAux {
                    mouseEnabled: false
                }
            }
        }

        //media and file column loader
        Column {
            spacing: CmnCfg.defaultMargin
            Loader {
                id: imageLoader
                sourceComponent: imageAttach ? image : undefined
                //image component
                Component {
                    id: image
                    AttachmentContent {}
                }
            }

            Loader {
                id: fileLoader
                sourceComponent: docAttach ? doc : undefined
                asynchronous: true
                // document component
                Component {

                    id: doc
                    FileAttachmentContent {}
                }
            }
        }

        //message body
        StandardTextEdit {
            id: messageBody
            maximumWidth: bubbleRoot.maxWidth
        }

        ElideHandler {}
        Loader {
            active: hasReactions

            sourceComponent: BubbleReacts {}
        }
    }
}
