import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

Rectangle {
    id: typingIndicator
    height: typingLoader.height
    width: parent.width
    color: typingLoader.active ? CmnCfg.palette.white : "transparent"
    property var conversationMembers
    Rectangle {
        visible: typingLoader.active
        anchors.top: parent.top
        width: parent.width
        height: 1
        color: CmnCfg.palette.medGrey
    }

    Connections {
        target: conversationMembers
        onNewTypingIndicator: {
            typingIndicator.__secondsSinceLastReset = 0
        }
    }

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {
            if (typingLoader.active)
                typingIndicator.__secondsSinceLastReset += 1
        }
    }

    property int __secondsSinceLastReset: 8
    property bool __aUserIsTyping: __secondsSinceLastReset < 7

    property string typeText
    Connections {
        target: conversationMembers
        onNewTypingIndicator: {
            typingIndicator.typeText = ""
            if (conversationMembers.typingMembers() === "") {
                typingLoader.active = false
                return
            }
            const typers = JSON.parse(conversationMembers.typingMembers())

            const num = typers.length
            const last = num - 1

            if (num <= 0) {
                typingLoader.active = false
                return
            }

            typers.forEach(function (item, index) {
                const typingUserName = item
                if (num === 1) {
                    typingIndicator.typeText += typingUserName + qsTr(
                                " is typing...")
                    return
                }

                if (num > 4) {
                    typingIndicator.typeText = "Several people are typing..."
                    return
                }
                if (num === 2 && index === 0) {
                    typingIndicator.typeText += typingUserName + qsTr(" and ")
                    return
                }

                if (index < last - 1) {
                    typingIndicator.typeText += typingUserName + ", "
                    return
                }

                if (index < last) {
                    typingIndicator.typeText += typingUserName + " and "
                    return
                }

                typingIndicator.typeText += typingUserName + qsTr(
                            " are typing...")
                return
            })
        }
    }

    Loader {
        id: typingLoader
        active: typingIndicator.__aUserIsTyping
        asynchronous: true

        height: CmnCfg.typeMargin

        width: parent.width
        anchors.bottom: parent.bottom
        sourceComponent: Label {
            topPadding: CmnCfg.microMargin
            leftPadding: CmnCfg.smallMargin
            id: typeLabel
            font.pixelSize: CmnCfg.chatTextSize
            font.italic: true
            text: typingIndicator.typeText
            font.family: CmnCfg.chatFont.name
        }
    }
}
