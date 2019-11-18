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

    property var blankTransition: Transition {
    }

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBar
        width: CmnCfg.padding
        policy: ScrollBar.AsNeeded
    }

    Component.onCompleted: {
        chatScrollBar.position = 1.0 + chatScrollBar.size
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
                id: chatRow
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
                readonly property string pfpUrl: outbound ? config.profilePicture : contactsModel.profilePictureById(
                                                                author)
                property alias highlight: bubbleActual.highlightItem

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
                        //mousearea handling jump behavior
                        jumpHandler.onClicked: {
                            convWindow.state = "jumpState"
                            convWindow.contentY = chatListView.itemAt(
                                        ownedConversation.indexById(
                                            replyId)).y - convWindow.height / 2
                            convWindow.returnToBounds()
                            convWindow.state = ""
                            replyHighlightAnimation.start()
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
                    highlight: match_status === 2
                    content: if (hasAttachments && dataSaved) {
                                 image
                             } else if (isReply) {
                                 reply
                             } else {
                                 std
                             }
                    ChatBubbleHover {
                    }
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
                    avatarHeight: 28
                }
            } //bubble wrapper
        } // Repeater
    } //singleton Col

    states: [
        State {
            name: "jumpState"
            PropertyChanges {
                target: cvPane
                rebound: blankTransition
            }

            PropertyChanges {
                target: chatScrollBar
                policy: ScrollBar.AlwaysOn
            }
        }
    ]
} // flickable
