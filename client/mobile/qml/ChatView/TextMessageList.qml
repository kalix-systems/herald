import QtQuick 2.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/qml/Common"
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/js/utils.mjs" as Utils

ListView {
    id: chatListView
    property Messages messageListModel

    highlightFollowsCurrentItem: false
    cacheBuffer: chatListView.height * 3

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBarInner
        width: CmnCfg.smallSpacer

        policy: ScrollBar.AsNeeded

        stepSize: 0.01
        minimumSize: 0.1
    }

    spacing: CmnCfg.margin
    model: messageListModel
    delegate: Column {
        id: containerCol
        readonly property string proxyBody: body

        // no receipt images for now
        property string proxyReceiptImage

        readonly property color userColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                                   author)]
        readonly property string timestamp: Utils.friendlyTimestamp(
                                                insertionTime)

        readonly property string authName: Herald.users.nameById(author)
        readonly property bool outbound: author === Herald.config.configId
        readonly property bool elided: body.length !== fullBody.length
        property var messageModelData: model

        anchors {
            right: outbound ? parent.right : undefined
            left: !outbound ? parent.left : undefined
            rightMargin: CmnCfg.margin * 2.0
            leftMargin: CmnCfg.margin * 2.0
        }

        //        Component {
        //            id: std
        //            CB.StandardBubble {
        //                body: proxyBody
        //                friendlyTimestamp: timestamp
        //                authorName: authName
        //                receiptImage: proxyReceiptImage
        //                authorColor: userColor
        //                elided: containerCol.elided
        //            }
        //        }
        CB.ChatBubble {
            defaultWidth: chatListView.width * 0.66
            messageModelData: containerCol.messageModelData
            convContainer: parent
        }
    }
}
