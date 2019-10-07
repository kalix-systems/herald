import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "../ChatBubble"
import "." as CVUtils
import "../common/utils.mjs" as Utils
import "../SideBar/ContactView.mjs" as CUtils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC
Flickable {
    property alias chatScrollBar: chatScrollBar
    property alias chatListView: chatListView
    id: cvPane

    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    contentHeight: textMessageCol.height

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBar
        width: QmlCfg.padding
    }

    Column {
        id: textMessageCol
        focus: true
        spacing: QmlCfg.padding
        topPadding: QmlCfg.padding
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
                readonly property string proxyReceiptImage: CUtils.receiptStatusSwitch(
                                                                0)
                readonly property string timestamp: Utils.friendlyTimestamp(
                                                        epochTimestampMs)
                readonly property bool outbound: author === config.configId
                // this is where scroll bar position needs to be set to instantiate in the right location
                Component.onCompleted: chatScrollBar.position = 1.0

                // column is most correct to resize for extra content
                anchors {
                    // This is okay as a ternary, the types are enforced by QML.
                    right: outbound ? parent.right : undefined
                    left: !outbound ? parent.left : undefined
                    rightMargin: QmlCfg.margin
                    leftMargin: QmlCfg.margin
                }
                rightPadding: QmlCfg.margin

                Component {
                    id: std
                    StandardBubble {
                        body: proxyBody
                        friendlyTimestamp: timestamp
                        authorName: outbound ? "" : author
                        receiptImage: proxyReceiptImage
                    }
                }

                Component {
                    id: reply
                    ReplyBubble {
                        body: proxyBody
                        friendlyTimestamp: timestamp
                        receiptImage: proxyReceiptImage
                    }
                }

                Component {
                    id: image
                    ImageBubble {
                        body: proxyBody
                        friendlyTimestamp: timestamp
                        receiptImage: proxyReceiptImage
                    }
                }

                ChatBubble {
                    ChatBubbleHover {}
                    radius: 10
                    maxWidth: cvPane.width * 0.66
                    color: outbound ? QmlCfg.palette.tertiaryColor : QmlCfg.avatarColors[3]
                    content: if (false) {
                                 image
                             } else if (op.bytelength === 32) {
                                 reply
                             } else {
                                 std
                             }
                }
            } //bubble wrapper
        } // Repeater
    } //singleton Col
} // flickable
