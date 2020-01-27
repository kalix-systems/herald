import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Entity" as Ent
import "qrc:/imports/js/utils.mjs" as Utils
import "../ChatView" as ChatView
import "../Common" as Common
import "./Controls" as Controls

// Layout & tap behavior for a conversation item in conversation list view
Rectangle {
    id: conversationItem

    property string itemTitle
    // the index corresponding to the visual color of this GroupBox
    property int colorCode: 0
    // path to the conversation's avatar image
    property string imageSource: ''
    property bool isGroup: false
    // whether this conversation is the "Note to Self" conversation
    property bool isNTS: false
    // TOOD(cmck) shouldn't need to pass this in
    property ConversationContent convoContent
    // true if the conversation contains no messages
    property bool isEmpty
    // true if the conversation is archived
    property bool isArchived
    // true if this message is selected (via long press)
    property bool isSelected: false
    // most recent message content to display in this item
    property var lastMsgDigest
    property alias ownedCV: ownedChatView
    property alias tapEnabled: tapHandler.enabled
    property alias optionsBar: optionsLoader.item

    //height: CmnCfg.convoHeight + optionsBar.height
    height: visible ? convoRectWrapper.height + optionsBar.height : 0

    // prevent animation spill over
    clip: true

    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
    }

    property int __secondsSinceLastReset: 0
    property int __typing: __secondsSinceLastReset < 8

    Connections {
        target: ContentMap.get(conversationData.conversationId).members
        onNewTypingIndicator: {
            conversationItem.__secondsSinceLastReset = 0

            convoRectangle.label.typeActive = true
        }
    }

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {
            conversationItem.__secondsSinceLastReset += 1
            if (!conversationItem.__typing) {
                convoRectangle.label.typeActive = false
            }
        }
    }

    Rectangle {
        id: convoRectWrapper
        height: CmnCfg.convoHeight
        width: parent.width

        Common.PlatonicRectangle {
            id: convoRectangle
            boxTitle: itemTitle
            boxColor: conversationItem.colorCode
            picture: imageSource
            isGroupPicture: conversationItem.isGroup
            isMessageResult: false

            labelComponent: Ent.ConversationLabel {
                convoTitle: itemTitle
                lastMsgDigest: conversationItem.convoContent.messages.lastMsgDigest
                isEmpty: conversationItem.isEmpty
                typeColor: CmnCfg.palette.medGrey
                typeSize: CmnCfg.defaultMargin
                receiptFill: CmnCfg.palette.black
            }
            z: CmnCfg.middleZ
        }

        Rectangle {
            id: selectionBackground
            anchors.fill: parent
            color: CmnCfg.palette.black
            opacity: .2
            visible: isSelected
        }

        // background item which gets manipulated
        // during the on tap animation
        Rectangle {
            id: splash
            width: 0
            height: width
            color: CmnCfg.palette.iconMatte
            opacity: 0
            radius: width
            transform: Translate {
                x: -splash.width / 2
                y: -splash.height / 2
            }
        }

        ParallelAnimation {
            id: splashAnim
            NumberAnimation {
                target: splash
                property: "width"
                duration: CmnCfg.units.longDuration
                easing.type: Easing.InOutQuad
                from: parent.width / 2
                to: parent.width * 2
            }
            NumberAnimation {
                target: splash
                property: "opacity"
                duration: CmnCfg.units.longDuration
                easing.type: Easing.InOutQuad
                from: 0.2
                to: 0
            }
            onRunningChanged: if (!running)
                                  mainView.push(ownedChatView)
        }

        Component {
            id: ownedChatView
            ChatView.ChatViewMain {
                property string stateName: "chat"
                headerTitle: itemTitle
                convoItem: conversationItem.conversationData
                convContent: convoContent
            }
        }

        TapHandler {
            id: tapHandler
            onTapped: {
                if (isSelected) {
                    optionsBar.deactivate()
                    isSelected = false
                } else {
                    splash.x = eventPoint.position.x
                    splash.y = eventPoint.position.y
                    // set the chat to the selected item
                    splashAnim.running = true
                    // callback implicity called at the end of the animation
                }
            }

            onLongPressed: {
                cvMainView.closeAllOptionsBars()
                isSelected = true
                optionsBar.activate()
            }
        }
    }

    Loader {
        id: optionsLoader
        active: tapEnabled
        anchors.top: convoRectWrapper.bottom
        height: active ? item.height : 0
        width: parent.width

        sourceComponent: Controls.ConvoOptionsBar {
            id: optionsBar
            width: parent.width
            isArchived: conversationItem.isArchived
        }
    }
}
