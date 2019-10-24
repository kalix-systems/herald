import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports"
import "." as CVUtils
import "../../foundation/js/utils.mjs" as Utils
import "../SideBar/js/ContactView.mjs" as CUtils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC
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
        spacing: CmnCfg.padding
        topPadding: CmnCfg.padding
        bottomPadding: CmnCfg.padding
        anchors {
            right: parent.right
            left: parent.left
        }

        Repeater {
            id: chatListView
            anchors.fill: parent
            model: ownedConversation
            delegate: Column {
                readonly property string proxyBody: body
                property string proxyReceiptImage: CUtils.receiptStatusSwitch(
                                                       receiptStatus)
                readonly property color userColor: CmnCfg.avatarColors[contactsModel.colorById(
                                                                           author)]
                readonly property string timestamp: Utils.friendlyTimestamp(
                                                        epochTimestampMs)
                readonly property string authName: outbound ? "" : contactsModel.nameById(
                                                                  author)
                readonly property bool outbound: author === config.configId

                // this is where scroll bar position needs to be set to instantiate in the right location
                Component.onCompleted: chatScrollBar.position = 1.0

                // column is most correct to resize for extra content
                anchors {
                    // This is okay as a ternary, the types are enforced by QML.
                    right: outbound ? parent.right : undefined
                    left: !outbound ? parent.left : undefined
                    rightMargin: CmnCfg.margin * 2.0
                    leftMargin: CmnCfg.margin * 2.0
                }
                rightPadding: CmnCfg.margin

                Component {
                    id: std
                    CB.StandardBubble {
                        body: proxyBody
                        friendlyTimestamp: timestamp
                        authorName: authName
                        receiptImage: proxyReceiptImage
                    }
                }

                Component {
                    id: reply
                    CB.ReplyBubble {
                        body: proxyBody
                        friendlyTimestamp: timestamp
                        opBody: ownedConversation.messageBodyById(op)
                        receiptImage: proxyReceiptImage
                        opName: ownedConversation.messageAuthorById(op)
                        opColor: CmnCfg.avatarColors[contactsModel.colorById(
                                                         opName)]
                        authorName: authName
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
                    }
                }

                CB.ChatBubble {
                    ChatBubbleHover {}
                    maxWidth: cvPane.width * 0.66
                    color: CmnCfg.palette.tertiaryColor
                    senderColor: userColor
                    content: if (hasAttachments && dataSaved) {
                                 image
                             } else if (isReply) {
                                 reply
                             } else {
                                 std
                             }
                }
            } //bubble wrapper
        } // Repeater
    } //singleton Col
} // flickable
