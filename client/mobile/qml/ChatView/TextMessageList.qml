import QtQuick 2.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/qml/Common"
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/js/utils.mjs" as Utils

ListView {
    id: chatListView
    property Messages messageListModel
    spacing: 0
    highlightFollowsCurrentItem: false
    cacheBuffer: chatListView.height * 3

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBarInner
        width: CmnCfg.smallSpacer

        policy: ScrollBar.AsNeeded

        stepSize: 0.01
        minimumSize: 0.1
    }

    model: messageListModel
    delegate: Column {
        id: containerCol
        readonly property string proxyBody: body
        spacing: 0

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

        anchors.left: parent.left
        anchors.right: parent.right
        bottomPadding: 0
        topPadding: 0

        CB.ChatBubble {
            defaultWidth: chatListView.width
            messageModelData: containerCol.messageModelData
            convContainer: parent
        }
    }
}
