import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "qrc:/imports"
import "../Common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports" as Imports

Page {
    id: moreInfoPopup
    // List of conversation member passed in upon push
    property var convoMembers
    // message data passed in upon push
    property var messageData
    // messages model passed in upon push
    property Messages ownedMessages: parent.ownedMessages
    // list of receipt-user objects. set on completion
    readonly property var receiptData: []
    property var outbound: messageData.author === Herald.config.configId

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    Component.onCompleted: {
        receiptData = JSON.parse(moreInfoPopup.messageData.userReceipts)
    }

    Flickable {
        anchors.fill: parent
        contentHeight: wrapperCol.height
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        Column {
            id: wrapperCol
            width: parent.width - CmnCfg.smallMargin * 2
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: CmnCfg.smallMargin
            topPadding: CmnCfg.smallMargin
            bottomPadding: CmnCfg.smallMargin

            CB.DefaultBubble {
                id: bubbleInfo
                convContainer: parent
                defaultWidth: parent.width
                width: parent.width
                messageModelData: moreInfoPopup.messageData
            }
            Label {
                id: senderHeader
                anchors.left: bubbleInfo.left
                text: "From:"
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            Item {
                id: author
                anchors.left: senderHeader.left
                height: CmnCfg.convoHeight
                width: parent.width
                Common.PlatonicRectangle {
                    boxTitle: messageData.authorName
                    boxColor: messageData.authorColor
                    picture: Utils.safeStringOrDefault(
                                 messageData.authorProfilePicture, "")
                    color: CmnCfg.palette.white
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
                }
            }

            Label {
                id: timeInfo
                anchors.left: author.left
                text: (outbound ? qsTr("Sent at: ") : qsTr(
                                      "Received at: ")) + Utils.userTime(
                          messageData.serverTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            Label {
                id: receiveInfo
                anchors.left: author.left
                text: "Received at: " + Utils.userTime(
                          messageData.insertionTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
                visible: !outbound
            }

            Label {
                id: expireInfo
                anchors.left: timeInfo.left
                visible: messageData.expirationTime !== undefined
                text: messageData.expirationTime
                      !== undefined ? "Expires at: " + Utils.userTime(
                                          messageData.expirationTime) : ""
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            Label {
                id: recHeader
                anchors.left: timeInfo.left
                text: "To:"
                font.family: CmnCfg.chatFont.name
                font.weight: Font.DemiBold
                color: CmnCfg.palette.black
            }

            ListView {
                height: contentHeight
                width: parent.width
                model: convoMembers
                interactive: false
                highlightFollowsCurrentItem: false
                currentIndex: -1
                delegate: Item {
                    height: visible ? CmnCfg.convoHeight : 0
                    width: 250
                    visible: memberData.userId !== messageData.author
                    property var memberData: model
                    Common.PlatonicRectangle {
                        boxTitle: memberData.name
                        boxColor: memberData.color
                        picture: Utils.safeStringOrDefault(memberData.picture,
                                                           "")
                        property MouseArea hoverHandler
                        color: CmnCfg.palette.white
                        labelComponent: Av.ConversationLabel {
                            contactName: memberData.name
                            lastBody: "@" + memberData.userId
                            labelColor: CmnCfg.palette.black
                            secondaryLabelColor: CmnCfg.palette.darkGrey
                            labelFontSize: CmnCfg.entityLabelSize
                        }

                        Button {
                            anchors.right: parent.right
                            id: receipt
                            icon.source: Utils.receiptCodeSwitch(
                                             receiptData[memberData.userId])
                            icon.height: 16
                            icon.width: 16
                            icon.color: CmnCfg.palette.iconMatte
                            padding: 0

                            anchors.verticalCenter: parent.verticalCenter
                            background: Item {}
                        }
                    }
                }
            }
            ToolButton {
                anchors.horizontalCenter: parent.horizontalCenter
                contentItem: Text {
                    text: qsTr("DELETE MESSAGE")
                    color: CmnCfg.palette.white
                    font.pixelSize: CmnCfg.headerFontSize
                    font.family: CmnCfg.chatFont.name
                }

                background: Rectangle {
                    color: CmnCfg.palette.alertColor
                }
                onClicked: {
                    moreInfoPopup.close()
                    messageInfoLoader.active = false
                    ownedMessages.deleteMessage(ownedMessages.indexById(
                                                    messageData.msgId))
                }
            }
        }
    }
}
