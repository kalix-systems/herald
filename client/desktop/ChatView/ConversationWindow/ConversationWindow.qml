import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick 2.13
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/Avatar"
import "." as CVUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "../../SideBar/js/ContactView.mjs" as CUtils

ListView {
    id: chatListView
    property alias chatScrollBar: chatScrollBarInner
    property alias chatListView: chatListView

    //this should be in here and not in the bubble because conversation window
    //needs access to it, add a separate animation to mobile
    //do not move this back into foundation
    property NumberAnimation highlightAnimation: NumberAnimation {
        id: bubbleHighlightAnimation
        property: "opacity"
        from: 1.0
        to: 0.0
        duration: 600
        easing.type: Easing.InCubic
    }

    // TODO this only clips because of highlight rectangles, figure out a way to
    // not use clip
    clip: true

    maximumFlickVelocity: 1500
    flickDeceleration: chatListView.height * 10

    onFlickStarted: focus = true

    highlightFollowsCurrentItem: false
    cacheBuffer: chatListView.height * 3

    Layout.maximumWidth: parent.width

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBarInner
        width: CmnCfg.padding

        policy: ScrollBar.AsNeeded
        hoverEnabled: true

        stepSize: 0.01
        minimumSize: 0.1
    }

    boundsBehavior: ListView.StopAtBounds
    boundsMovement: Flickable.StopAtBounds

    model: ownedConversation

    Component.onCompleted: {
        ownedConversation.setElisionLineCount(38)
        ownedConversation.setElisionCharCount(38 * 40)
        ownedConversation.setElisionCharsPerLine(40)
        positionViewAtEnd()
    }

    delegate: Row {
        id: chatRow
        readonly property string proxyBody: body
        property string proxyReceiptImage: CUtils.receiptStatusSwitch(
                                               receiptStatus)
        readonly property color userColor: CmnCfg.avatarColors[herald.users.colorById(
                                                                   author)]
        readonly property string timestamp: Utils.friendlyTimestamp(
                                                insertionTime)

        readonly property bool outbound: author === herald.config.configId

        readonly property string authName: outbound ? herald.config.name : herald.users.nameById(
                                                          author)
        readonly property string pfpUrl: outbound ? herald.config.profilePicture : herald.users.profilePictureById(
                                                        author)
        property alias highlight: bubbleActual.highlightItem
        property bool elided: body.length !== fullBody.length

        anchors {
            right: outbound ? parent.right : undefined
            left: !outbound ? parent.left : undefined
            rightMargin: CmnCfg.margin
            leftMargin: CmnCfg.smallMargin
        }

        spacing: CmnCfg.margin
        bottomPadding: isTail ? CmnCfg.mediumMargin / 2 : CmnCfg.smallMargin / 2
        topPadding: isHead ? CmnCfg.mediumMargin / 2 : CmnCfg.smallMargin / 2

        Component {
            id: std
            CB.StandardBubble {
                body: proxyBody
                friendlyTimestamp: timestamp
                authorName: authName
                receiptImage: proxyReceiptImage
                authorColor: userColor
                elided: chatRow.elided
            }
        }

        Component {
            id: reply
            CB.ReplyBubble {
                body: proxyBody
                friendlyTimestamp: timestamp
                receiptImage: proxyReceiptImage
                authorName: authName
                authorColor: userColor
                replyId: opMsgId
                elided: chatRow.elided
                //mousearea handling jump behavior
                jumpHandler.onClicked: {
                    const msgIndex = ownedConversation.indexById(replyId)
                    if (msgIndex < 0)
                        return

                    const window = convWindow

                    window.positionViewAtIndex(msgIndex, ListView.Center)
                    window.highlightAnimation.target = window.itemAtIndex(
                                msgIndex).highlight
                    window.highlightAnimation.start()
                }
            }
        }

        Component {
            id: image
            CB.ImageBubble {
                body: proxyBody
                friendlyTimestamp: timestamp
                receiptImage: proxyReceiptImage
                authorName: authName
                mediaAttachments: ownedConversation.mediaAttachments
                documentAttachments: ownedConversation.documentAttachments
                authorColor: userColor
                elided: chatRow.elided
            }
        }

        AvatarMain {
            iconColor: userColor
            initials: authName[0].toUpperCase()
            opacity: isTail && !outbound ? 1 : 0
            size: 28
            anchors {
                bottom: parent.bottom
                margins: CmnCfg.margin
                bottomMargin: parent.bottomPadding
            }
            z: 10
            pfpPath: parent.pfpUrl
            avatarHeight: 28
        }

        CB.ChatBubble {
            id: bubbleActual
            maxWidth: chatListView.width * 0.66
            color: CmnCfg.palette.lightGrey
            senderColor: userColor
            convContainer: convWindow
            highlight: matchStatus === 2
            content: if (ownedConversation.mediaAttachments.length !== 0
                             && ownedConversation.documentAttachments.length !== 0) {
                         image
                         //reply types: 0 not reply, 1 dangling, 2 known reply
                     } else if (replyType > 0) {
                         reply
                     } else {
                         std
                     }
            ChatBubbleHover {}
        }

        AvatarMain {
            iconColor: userColor
            initials: authName[0].toUpperCase()
            opacity: isTail && outbound ? 1 : 0
            size: 28
            anchors {
                bottom: parent.bottom
                margins: CmnCfg.margin
                bottomMargin: parent.bottomPadding
            }
            z: 10
            pfpPath: parent.pfpUrl
            avatarHeight: 28
        }
    }
}
