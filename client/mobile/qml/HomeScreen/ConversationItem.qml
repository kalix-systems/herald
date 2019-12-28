import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import "../ChatView" as ChatView
import "../Common" as Common

// Layout & tap behavior for a conversation item in conversation list view
Rectangle {
    id: contactItem

    property string convoTitle
    // the index corresponding to the visual color of this GroupBox
    property int colorCode: 0
    // path to the conversation's avatar image
    property string imageSource: ''
    property bool isGroup: false
    // asdf
    property ConversationContent convContent: null

    height: entityBlock.height
    color: CmnCfg.palette.white

    // prevent animation spill over
    clip: true

    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
    }

    Common.EntityBlock {
        id: entityBlock
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.rightMargin: CmnCfg.smallMargin

        entityName: convoTitle
        subLabelText: convContent.messages.lastBody
        timestamp: convContent.messages.isEmpty ? "" : Utils.friendlyTimestamp(
                                                      convContent.messages.lastTime)
        lastReceipt: convContent.messages.lastStatus
                     === undefined ? 0 : convContent.messages.lastStatus
        color: CmnCfg.avatarColors[contactItem.colorCode]
        isGroup: contactItem.isGroup
        pfpPath: imageSource
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
            ownedMessages: contactItem.convContent.messages
            headerTitle: convoTitle
        }
    }

    TapHandler {
        onTapped: {
            splash.x = eventPoint.position.x
            splash.y = eventPoint.position.y
            // set the chat to the selected item
            splashAnim.running = true
            // callback implicity called at the end of the animation
        }
    }
}
