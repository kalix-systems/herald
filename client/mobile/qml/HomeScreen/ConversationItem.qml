import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils
import "../ChatView" as ChatView
import "../Common" as Common

Rectangle {
    id: contactItem

    // the index corresponding to the visual color of this GroupBox
    property int colorCode: 0
    //property string proxyTitle: title
    property ConversationContent convContent: null

    property real topTextMargin: 2
    property real bottomTextMargin: 6

    height: CmnCfg.avatarSize
    color: CmnCfg.palette.white

    // prevent animation spill over
    clip: true
    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
        //fill: parent
    }

    Common.EntityBlock {
        entityName: title
         subLabelText: convContent.messages.lastBody
         timestamp: convContent.messages.isEmpty ? "" : Utils.friendlyTimestamp(
                                                       convContent.messages.lastTime)
        lastReceipt: convContent.messages.lastStatus
                     === undefined ? 0 : convContent.messages.lastStatus
        color: CmnCfg.avatarColors[contactItem.colorCode]
    }

//    Avatar {
//        id: itemAvatar
//        anchors {
//            left: parent.left
//            verticalCenter: parent.verticalCenter
//            leftMargin: CmnCfg.smallMargin
//        }
//        color: CmnCfg.avatarColors[colorCode]
//        initials: Utils.initialize(title)
//        diameter: CmnCfg.units.dp(48)
//        isGroup: false
//        // TODO pfpPath
//    }

//    ConversationLabel {
//        anchors {
//            left: itemAvatar.right
//            right: parent.right
//            top: parent.top
//            bottom: parent.bottom
//            leftMargin: CmnCfg.defaultMargin
//            rightMargin: CmnCfg.defaultMargin
//            topMargin: topTextMargin
//            bottomMargin: bottomTextMargin

//        }

//        contactName: title
//        lastBody: convContent.messages.lastBody
//        lastTimestamp: convContent.messages.isEmpty ? "" : Utils.friendlyTimestamp(
//                                                          convContent.messages.lastTime)
//        lastReceipt: convContent.messages.lastStatus
//                     === undefined ? 0 : convContent.messages.lastStatus
//        labelFontSize: CmnCfg.labelSize
//    }

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
            ownedMessages: contactItem.convContent.messages
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
