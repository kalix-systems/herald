import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "../Entity"
import "./ReplyBubble"
import "../js/utils.mjs" as Utils

Rectangle {
    id: bubbleRoot

    property real defaultWidth
    property bool elided: body.length !== messageModelData.fullBody.length
    property bool expanded: false
    property bool outbound: author === Herald.config.configId
    property Item convContainer
    property var messageModelData

    property alias imageClickedCallBack: imageLoader.imageClickedCallBack
    property Messages ownedConversation
    property alias highlightItem: bubbleHighlight
    readonly property color bubbleColor: CmnCfg.palette.lightGrey
    readonly property bool highlight: messageModelData.matchStatus === 2

    readonly property string body: messageModelData.body
    readonly property string authorId: messageModelData.author
    readonly property string authorName: outbound ? UserMap.get(
                                                        Herald.config.configId).name : UserMap.get(
                                                        messageModelData.author).name

    readonly property string medAttachments: messageModelData.mediaAttachments
    readonly property string fullMedAttachments: messageModelData.fullMediaAttachments
    readonly property string documentAttachments: messageModelData.docAttachments
    readonly property bool imageAttach: medAttachments.length !== 0
    readonly property bool docAttach: documentAttachments.length !== 0

    readonly property var replyId: messageModelData.opMsgId
    readonly property bool reply: messageModelData.replyType > 0

    readonly property bool isHead: messageModelData.isHead
    readonly property bool isTail: messageModelData.isTail

    readonly property real maxWidth: defaultWidth * 0.72
    property string friendlyTimestamp: outbound ? Utils.friendlyTimestamp(
                                                      messageModelData.insertionTime) : Utils.friendlyTimestamp(
                                                      messageModelData.serverTime)

    property string timerIcon: expirationTime !== undefined ? Utils.timerIcon(
                                                                  expirationTime,
                                                                  insertionTime) : ""
    readonly property string receiptImage: outbound ? Utils.receiptCodeSwitch(
                                                          messageModelData.receiptStatus) : ""
    //not readonly in order to be assigned tied to convo color for pairwise
    property color authorColor: CmnCfg.avatarColors[UserMap.get(
                                                        messageModelData.author).userColor]

    readonly property string pfpUrl: outbound ? UserMap.get(
                                                    Herald.config.configId).profilePicture : UserMap.get(
                                                    messageModelData.author).profilePicture
    // true if this message's options menu is open (mobile only)
    property bool isSelected: false
    property bool hoverHighlight: isSelected || false
    property alias expireInfo: expireInfo
    property int bubbleIndex
    property bool moreInfo: false
    property bool aux: false
    property var convoExpiration
    property MouseArea hitbox

    property bool sameExpiration: {
        if (messageModelData.expirationTime === undefined) {
            return convoExpiration === 0
        }
        return Utils.sameExp(messageModelData.insertionTime,
                             messageModelData.expirationTime, convoExpiration)
    }
    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {
            friendlyTimestamp
                    = (outbound ? Utils.friendlyTimestamp(
                                      messageModelData.insertionTime) : Utils.friendlyTimestamp(
                                      messageModelData.serverTime))
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

    height: contentRoot.height
    width: defaultWidth

    color: CmnCfg.palette.white

    Rectangle {
        anchors.top: parent.top
        width: parent.width
        height: 1
        color: CmnCfg.palette.medGrey
        visible: isHead
        z: accent.z + 1
    }

    Highlight {
        id: bubbleHighlight
        z: bubbleRoot.z + 1
    }

    Avatar {
        id: avatar
        color: authorColor
        initials: authorName[0].toUpperCase()
        size: CmnCfg.headerAvatarSize
        visible: isHead ? true : false
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

    Button {
        id: receipt
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.bottomMargin: (bubbleIndex === 0) ? CmnCfg.smallMargin
                                                    + (CmnCfg.typeMargin - CmnCfg.microMargin
                                                       * 1.5) : CmnCfg.smallMargin
        anchors.rightMargin: CmnCfg.smallMargin

        icon.source: receiptImage
        icon.height: CmnCfg.units.dp(14)
        icon.width: CmnCfg.units.dp(14)
        icon.color: CmnCfg.palette.darkGrey
        padding: 0
        background: Item {}
    }

    BubbleExpireInfo {
        id: expireInfo
        visible: isHead // || !sameExpiration
    }

    Column {
        z: highlight.z + 1
        id: contentRoot
        anchors.left: accent.right
        // all messages are un-expanded on completion
        Component.onCompleted: bubbleRoot.expanded = false

        spacing: CmnCfg.smallMargin
        topPadding: isHead ? CmnCfg.smallMargin : CmnCfg.smallMargin
        leftPadding: CmnCfg.smallMargin
        bottomPadding: {
            if (bubbleIndex === 0) {
                return CmnCfg.defaultMargin + (CmnCfg.typeMargin - CmnCfg.microMargin * 1.5)
            }

            isTail ? CmnCfg.defaultMargin : CmnCfg.smallMargin
        }

        BubbleLabel {
            visible: isHead
            timestamp: friendlyTimestamp
            id: authorLabel
        }

        //reply bubble loader
        Loader {

            sourceComponent: {
                if (!reply)
                    return undefined

                if (replyType === 1) {
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
                ReplyHybrid {}
            }

            // reply bubble if there is doc file content
            Component {
                id: replyDanglingContent
                ReplyDangling {}
            }

            // reply bubble if there is doc file content
            Component {
                id: replyDocContent
                ReplyDoc {}
            }

            // reply media bubble if there is media file content
            Component {
                id: replyMediaContent
                ReplyImage {}
            }

            // reply bubble if there is no doc file content
            Component {
                id: replyContent
                ReplyText {}
            }

            // reply bubble if it is an aux message
            Component {
                id: replyAux
                ReplyAux {}
            }
        }

        // media and file column loader
        Column {
            spacing: CmnCfg.defaultMargin
            Loader {
                property var imageClickedCallBack
                id: imageLoader
                sourceComponent: imageAttach ? image : undefined
                // image component
                Component {
                    id: image
                    AttachmentContent {

                        imageClickedCallBack: imageLoader.imageClickedCallBack
                    }
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

        // message body
        StandardTextEdit {
            id: messageBody
            maximumWidth: bubbleRoot.maxWidth
        }

        ElideHandler {}

        Loader {
            active: messageModelData.reactions.length > 0

            sourceComponent: BubbleReacts {}
        }
    }
}
