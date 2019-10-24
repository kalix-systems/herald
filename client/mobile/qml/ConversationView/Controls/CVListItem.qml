import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Avatar"
import "../js/CVViewUtils.js" as CVJS

Rectangle {
    id: contactItem

    // the group name or displayName of the conversation
    property string contactName
    // the previous message of the conversation, or the empty string
    property string body
    // the previous latest human readable timestamp, or the empty string
    property string timestamp
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: 0
    // the index corresponding to the visual color of this GroupBox
    property int colorCode: 0
    // the owned conversation model corresponding to this conversation id
    // may be reset upon forking a conversation
    property Messages ownedMessages

    height: CmnCfg.avatarSize
    color: CmnCfg.palette.mainColor

    // prevent animation spill over
    clip: true
    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
    }

    AvatarMain {
        id: avatar
        iconColor: CmnCfg.avatarColors[colorCode]
        anchors.verticalCenter: parent.verticalCenter
        initials: CVJS.initialize(title)
        labelComponent: ConversationLabel {
            contactName: title
            lastBody: lastBody
            lastTimestamp: lastTimestamp
            lastReceipt: lastReceipt
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
                appState.state = "chat"
            }
        }
    }

    TapHandler {
        onTapped: {
            splash.x = eventPoint.position.x
            splash.y = eventPoint.position.y
            splashAnim.running = true
            // set the chat to the selected item
            appState.chatMain.headerTitle = title
            appState.chatMain.ownedMessages = contactItem.ownedMessages
            // callback implicity called at the end of the animation
        }
    }
}
