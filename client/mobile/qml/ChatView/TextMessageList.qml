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

    // this is set to a higher value in `Component.onCompleted`
    // but is set to `0` here to improve initial load times
    cacheBuffer: 0
    Component.onCompleted: cacheBuffer = chatListView.height * 5

    // Note: we load the list view from the bottom up to make
    // scroll behavior more predictable
    verticalLayoutDirection: ListView.BottomToTop

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBarInner
        width: CmnCfg.smallSpacer
        policy: ScrollBar.AsNeeded
        stepSize: 0.01
        minimumSize: 0.1
    }

    model: messageListModel
    // TODO: Delegate should just be the ChatBubble
    delegate: Column {
        id: containerCol
        spacing: 0

        // no receipt images for now
        readonly property bool outbound: author === Herald.config.configId
        readonly property bool elided: body.length !== fullBody.length
        property var messageModelData: model

        anchors.left: parent.left
        anchors.right: parent.right
        bottomPadding: 0
        topPadding: 0

        CB.ChatBubble {
            id: chatBubble
            defaultWidth: chatListView.width
            messageModelData: containerCol.messageModelData
            convContainer: parent
            property Component infoPage: Component {
                InfoPage {
                    referredChatBubble: chatBubble
                }
            }

            MessageMouseArea {
                cb: parent
                dropdown: dropdown
                anchors.fill: parent
            }
        }

        OptionsDropdown {
            id: dropdown
            cb: chatBubble
        }
    }
}
