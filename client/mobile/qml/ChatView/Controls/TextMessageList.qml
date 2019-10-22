import QtQuick 2.12
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB

Column {
    id: textMessageCol
    property Messages model: value

    topPadding: QmlCfg.padding
    bottomPadding: QmlCfg.padding
    anchors {
        right: parent.right
        left: parent.left
    }

    Repeater {
        id: chatListView
        anchors.fill: parent
        model: parent.model

        delegate: Column {
            readonly property string proxyBody: body
            property string proxyReceiptImage: CUtils.receiptStatusSwitch(
                                                   receiptStatus)
            readonly property color userColor: QmlCfg.avatarColors[contactsModel.colorById(
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
                rightMargin: QmlCfg.margin * 2.0
                leftMargin: QmlCfg.margin * 2.0
            }
            rightPadding: QmlCfg.margin

            Component {
                id: std
                CB.StandardBubble {
                    body: proxyBody
                    friendlyTimestamp: timestamp
                    authorName: authName
                    receiptImage: proxyReceiptImage
                }
            }

            CB.ChatBubble {
                maxWidth: cvPane.width * 0.66
                color: QmlCfg.palette.tertiaryColor
                senderColor: userColor
                content: std
            }
        } //bubble wrapper
    } // Repeater
} //singleton Col