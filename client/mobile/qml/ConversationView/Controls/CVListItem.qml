import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils
import "../../ChatView" as ChatView
import "../js/CVViewUtils.js" as CVJS

Rectangle {
    id: contactItem

    // the index corresponding to the visual color of this GroupBox
    property int colorCode: 0
    property string proxyTitle: title

    height: CmnCfg.avatarSize
    color: CmnCfg.palette.mainColor

    // prevent animation spill over
    clip: true
    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
    }

    Rectangle {

        anchors {
            fill: parent
            rightMargin: CmnCfg.units.dp(12)
            leftMargin: CmnCfg.units.dp(12)
        }

        AvatarMain {
            id: avatar
            iconColor: CmnCfg.avatarColors[colorCode]
            anchors.verticalCenter: parent.verticalCenter
            initials: CVJS.initialize(title)
            size: CmnCfg.units.dp(32)

            anchors {
                right: parent.right
                left: parent.left
            }

            labelComponent: ConversationLabel {
                contactName: title
                lastBody: ownedMessages.lastBody
                lastTimestamp: Utils.friendlyTimestamp(
                                   ownedMessages.lastEpochTimestampMs)
                lastReceipt: ownedMessages.lastStatus === undefined ? 0 : ownedMessages.lastStatus
            }
        }
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
        onRunningChanged: {
            if (!!!running) {
                mainView.push(ownedChatView)
            }
        }
    }

    Component {
        id: ownedChatView
        ChatView.ChatViewMain {
            ownedMessages: contactItem.ownedMessages
            headerTitle: proxyTitle
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
