import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick 2.13
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/Avatar"
import "." as CVUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "../SideBar/js/ContactView.mjs" as CUtils

Flickable {
    id: cvPane
    property alias chatScrollBar: chatScrollBar
    property alias chatListView: chatListView
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    contentHeight: textMessageCol.height

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBar
        width: CmnCfg.padding
    }

    Component.onCompleted: {
        chatScrollBar.position = 1.0
    }

    Column {
        id: textMessageCol
        focus: true

        //   spacing: CmnCfg.smallMargin
        anchors {
            right: parent.right
            left: parent.left
        }

        Repeater {
            id: chatListView
            anchors.fill: parent
            model: ownedConversation

            delegate: Row {
                readonly property string proxyBody: body
                property string proxyReceiptImage: CUtils.receiptStatusSwitch(
                                                       receiptStatus)
                readonly property color userColor: CmnCfg.avatarColors[contactsModel.colorById(
                                                                           author)]
                readonly property string timestamp: Utils.friendlyTimestamp(
                                                        epochTimestampMs)

                readonly property bool outbound: author === config.configId

                readonly property string authName: outbound ? config.name : contactsModel.nameById(
                                                                  author)

                spacing: CmnCfg.margin
                readonly property string pfpUrl: outbound ? config.profilePicture
                                                          : contactsModel.profilePictureById(author)
                // column is most correct to resize for extra content
                anchors {
                    right: outbound ? parent.right : undefined
                    left: !outbound ? parent.left : undefined
                    rightMargin: CmnCfg.margin
                    leftMargin: CmnCfg.smallMargin
                }
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
                        replyId: op
                    }
                }

                Component {
                    id: image
                    CB.ImageBubble {
                        body: proxyBody
                        friendlyTimestamp: timestamp
                        receiptImage: proxyReceiptImage
                        authorName: authName
                        messageAttachments: Attachments {
                            msgId: messageId
                        }
                        authorColor: userColor
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
                    pfpPath: parent.pfpUrl
                }

                CB.ChatBubble {
                    id: bubbleActual
                    maxWidth: cvPane.width * 0.66
                    color: CmnCfg.palette.paneColor
                    senderColor: userColor
                    content: if (hasAttachments && dataSaved) {
                                 image
                             } else if (isReply) {
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
                    pfpPath: parent.pfpUrl
                }
            } //bubble wrapper
        } // Repeater
    } //singleton Col
} // flickable
