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
    property bool elided: body.length !== messageModelData.fullBody.length
    property bool expanded: false
    property bool outbound: messageModelData.author === Herald.config.configId
    property Item convContainer
    property var messageModelData

    property alias highlightItem: bubbleHighlight
    readonly property color bubbleColor: CmnCfg.palette.lightGrey
    readonly property bool highlight: messageModelData.matchStatus === 2

    readonly property string body: messageModelData.body
    readonly property string authorId: messageModelData.author
    readonly property string authorName: messageModelData.authorName

    readonly property string medAttachments: messageModelData.mediaAttachments
    readonly property string fullMedAttachments: messageModelData.fullMediaAttachments
    readonly property string documentAttachments: messageModelData.docAttachments
    readonly property bool imageAttach: medAttachments.length !== 0
    readonly property bool docAttach: documentAttachments.length !== 0

    readonly property var replyId: messageModelData.opMsgId
    readonly property bool reply: messageModelData.replyType > 0

    readonly property bool isHead: messageModelData.isHead
    readonly property bool isTail: messageModelData.isTail
    readonly property bool hasReactions: messageModelData.reactions.length > 0

    readonly property real maxWidth: defaultWidth * 0.75
    property string friendlyTimestamp: Utils.friendlyTimestamp(
                                           messageModelData.insertionTime)

    property string timerIcon: messageModelData.expirationTime
                               !== undefined ? Utils.timerIcon(
                                                   messageModelData.expirationTime,
                                                   messageModelData.insertionTime) : ""
    readonly property string receiptImage: outbound ? Utils.receiptCodeSwitch(
                                                          messageModelData.receiptStatus) : ""
    readonly property color authorColor: CmnCfg.avatarColors[messageModelData.authorColor]

    readonly property string pfpUrl: messageModelData.authorProfilePicture
    property bool hoverHighlight: false
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
        initials: authorName[0].toUpperCase()
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
        icon.height: 16
        icon.width: 16
        icon.color: CmnCfg.palette.iconMatte
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
