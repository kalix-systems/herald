import QtQuick 2.12
import LibHerald 1.0
import "qrc:/qml/Common"
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/js/utils.mjs" as Utils

Column {
    id: textMessageCol
    property Messages model
    spacing: CmnCfg.margin

    Repeater {
        id: chatListView
        model: parent.model
        delegate: Column {
            readonly property string proxyBody: body

            // no receipt images for now
            property string proxyReceiptImage

            readonly property color userColor: CmnCfg.avatarColors[herald.users.colorById(
                                                                       author)]
            readonly property string timestamp: Utils.friendlyTimestamp(
                                                    insertionTime)

            readonly property string authName: herald.users.nameById(
                                                              author)
            readonly property bool outbound: author === herald.config.configId

            // column is most correct to resize for extra content
            anchors {
                // This is okay as a ternary, the types are enforced by QML.
                right: outbound ? parent.right : undefined
                left: !outbound ? parent.left : undefined
                rightMargin: CmnCfg.margin * 2.0
                leftMargin: CmnCfg.margin * 2.0
            }

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

            CB.ChatBubble {
                maxWidth: textMessageCol.width * 0.66
                color: CmnCfg.palette.lightGrey
                senderColor: userColor
                content: std
                convContainer: parent
            }
        } //bubble wrapper
    } // Repeater
} //singleton Col
