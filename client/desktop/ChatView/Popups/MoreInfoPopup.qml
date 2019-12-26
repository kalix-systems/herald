import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports"
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/ChatBubble" as CB

Popup {
    id: moreInfoPopup
    property var convoMembers: parent.convoMembers
    property var messageData: parent.messageData
    property var receiptData

    height: parent.height
    width: parent.width
    anchors.centerIn: parent
    onClosed: messageInfoLoader.active = false

    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }

    Component.onCompleted: {
        receiptData = JSON.parse(moreInfoPopup.messageData.userReceipts)
    }

    CB.DefaultBubble {
        id: bubbleInfo
        convContainer: parent
        defaultWidth: parent.width
        width: parent.width
        messageModelData: moreInfoPopup.messageData
        anchors.top: parent.top

        IconButton {
            anchors.top: parent.top
            anchors.right: parent.right
            anchors.margins: CmnCfg.microMargin
            id: xIcon
            source: "qrc:/x-icon.svg"
            icon.height: 26
            icon.width: 26
            fill: CmnCfg.palette.black
            z: parent.z + 1
            onClicked: {
                messageInfoLoader.active = false
                moreInfoPopup.close()
            }
        }
    }
    Label {
        id: senderHeader
        anchors.top: bubbleInfo.bottom
        anchors.topMargin: CmnCfg.smallMargin
        anchors.left: bubbleInfo.left
        text: "From:"
        font.family: CmnCfg.chatFont.name
        font.weight: Font.DemiBold
        color: CmnCfg.palette.black
    }

    Item {
        id: author
        anchors.top: senderHeader.bottom
        anchors.topMargin: CmnCfg.smallMargin
        anchors.left: senderHeader.left
        height: CmnCfg.convoHeight
        width: parent.width
        Common.PlatonicRectangle {

            boxTitle: messageData.authorName
            boxColor: messageData.authorColor
            picture: Utils.safeStringOrDefault(
                         messageData.authorProfilePicture, "")
            color: CmnCfg.palette.lightGrey
            labelComponent: Av.ConversationLabel {
                contactName: messageData.authorName
                lastBody: "@" + messageData.author
                labelColor: CmnCfg.palette.black
                secondaryLabelColor: CmnCfg.palette.darkGrey
                labelFontSize: CmnCfg.entityLabelSize
            }
            MouseArea {
                id: hoverHandler
            }

            states: []
        }
    }

    Label {
        id: timeInfo
        anchors.top: author.bottom
        anchors.topMargin: CmnCfg.smallMargin
        anchors.left: author.left
        text: "At: " + Utils.userTime(messageData.insertionTime)
        font.family: CmnCfg.chatFont.name
        font.weight: Font.DemiBold
        color: CmnCfg.palette.black
    }

    Label {
        id: recHeader
        anchors.top: timeInfo.bottom
        anchors.topMargin: CmnCfg.smallMargin
        anchors.left: timeInfo.left
        text: "To:"
        font.family: CmnCfg.chatFont.name
        font.weight: Font.DemiBold
        color: CmnCfg.palette.black
    }

    ListView {
        height: contentHeight
        width: parent.width
        anchors.top: recHeader.bottom
        anchors.topMargin: CmnCfg.smallMargin
        model: convoMembers
        highlightFollowsCurrentItem: false
        currentIndex: -1
        delegate: Item {
            height: visible ? CmnCfg.convoHeight : 0
            width: parent.width
            visible: memberData.userId !== messageData.author
            property var memberData: model
            Common.PlatonicRectangle {
                boxTitle: memberData.name
                boxColor: memberData.color
                picture: Utils.safeStringOrDefault(memberData.picture, "")
                color: CmnCfg.palette.lightGrey
                labelComponent: Av.ConversationLabel {
                    contactName: memberData.name
                    lastBody: "@" + memberData.userId
                    labelColor: CmnCfg.palette.black
                    secondaryLabelColor: CmnCfg.palette.darkGrey
                    labelFontSize: CmnCfg.entityLabelSize
                }

                Button {
                    id: receipt
                    icon.source: Utils.receiptCodeSwitch(
                                     receiptData[memberData.userId])
                    icon.height: 16
                    icon.width: 16
                    icon.color: CmnCfg.palette.iconMatte
                    padding: 0
                    anchors.right: parent.right
                    anchors.rightMargin: CmnCfg.smallMargin

                    anchors.verticalCenter: parent.verticalCenter
                    background: Item {}
                }
                MouseArea {
                    id: hoverHandler
                }

                states: []
            }
        }
    }
}
