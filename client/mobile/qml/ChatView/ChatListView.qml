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
    property alias scrollBar: chatScrollBarInner

    property NumberAnimation highlightAnimation: NumberAnimation {
        id: bubbleHighlightAnimation
        property: "opacity"
        from: 0.4
        to: 0.0
        duration: 800
        easing.type: Easing.InCubic
    }

    // Used to close other OptionsDropdowns when a new one is opened
    signal closeDropdown

    // this is set to a higher value in `Component.onCompleted`
    // but is set to `0` here to improve initial load times
    cacheBuffer: 0
    Component.onCompleted: {
        cacheBuffer = chatListView.height * 5

        ownedMessages.setElisionLineCount(20)
        ownedMessages.setElisionCharCount(20 * 30)
        ownedMessages.setElisionCharsPerLine(30)
        if (chatListView.contentHeight < chatListView.height) {
            chatListView.height = chatListView.contentHeight
        }
        appRouter.activeChatView = chatListView
    }
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
    boundsBehavior: ListView.StopAtBounds
    boundsMovement: Flickable.StopAtBounds

    Connections {
        target: ownedMessages
        onRowsInserted: {
            chatScrollBarInner.position = 1.0
        }
    }

    Popup {
        id: emojiPopup
        width: parent.contentWidth
        height: reactPopup.height
        property var chatBubble
        anchors.centerIn: parent
        property alias reactPopup: emoKeysPopup
        background: Item {}
        modal: true

        onClosed: {
            reactPopup.active = false
        }
        Loader {
            id: emoKeysPopup
            active: false
            height: active ? CmnCfg.units.dp(200) : 0
            width: parent.width
            sourceComponent: EmojiPicker {}
        }
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
        property var highlightItem: bubbleLoader.item.highlightItem
        anchors.left: parent.left
        anchors.right: parent.right
        bottomPadding: 0
        topPadding: 0
        Loader {
            id: bubbleLoader
            property var modelData: model
            sourceComponent: model.auxData.length === 0 ? msgBubble : auxBubble
            width: parent.width
            height: active ? item.height : undefined
            property bool isAux: model.auxData.length !== 0

            MessageMouseArea {
                cb: parent.item
                dropdown: optionsDropdown
                anchors.fill: parent
            }

            Component {
                id: msgBubble
                CB.ChatBubble {
                    id: bubbleActual
                    defaultWidth: chatListView.width
                    messageModelData: containerCol.messageModelData
                    convContainer: parent
                    convoExpiration: convoItem.expirationPeriod
                    ownedConversation: ownedMessages
                    bubbleIndex: index
                    property Component infoPage: Component {
                        InfoPage {
                            members: convContent.members
                            messageData: bubbleActual.messageModelData
                        }
                    }
                }
            }

            Component {
                id: auxBubble
                CB.AuxBubble {

                    id: bubbleActual
                    auxData: JSON.parse(model.auxData)
                    messageModelData: model
                    width: parent.width
                    defaultWidth: chatListView.width
                    bubbleIndex: index
                    ownedConversation: ownedMessages
                }
            }
        }

        OptionsDropdown {
            id: optionsDropdown
            cb: bubbleLoader.item
        }
    }
}
