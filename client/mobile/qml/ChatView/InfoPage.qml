import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "qrc:/imports"
import "../Common" as Common
import "qrc:/imports/Entity" as Ent
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/ChatBubble"
import "qrc:/imports" as Imports

Page {
    id: moreInfoPopup
    readonly property Component headerComponent: MessageInfoHeader {}

    property var messageData
    // members of the conversation content
    property var members
    readonly property string stateName: "info"
    // list of receipt-user objects. set on completion
    property var receiptData: []

    Component.onCompleted: {
        receiptData = JSON.parse(messageData.userReceipts)
    }

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    Flickable {
        anchors.fill: parent
        anchors.bottomMargin: CmnCfg.largeMargin
        contentHeight: pageContent.height
        boundsBehavior: Flickable.StopAtBounds

        Column {
            id: pageContent
            width: parent.width
            padding: CmnCfg.defaultMargin
            spacing: CmnCfg.defaultMargin

            DefaultBubble {
                defaultWidth: parent.width
                convContainer: parent
                messageModelData: messageData
                width: parent.width
                anchors.horizontalCenter: parent.horizontalCenter
                moreInfo: false
            }

            Column {
                Label {
                    id: fromLabel
                    text: qsTr("From: ")

                    font {
                        family: CmnCfg.chatFont.name
                        pixelSize: CmnCfg.chatTextSize
                        weight: Font.Medium
                    }
                }

                Item {
                    id: author
                    anchors.left: fromLabel.left
                    height: CmnCfg.convoHeight
                    width: parent.width
                    Common.PlatonicRectangle {

                        boxTitle: messageData.authorName
                        boxColor: messageData.authorColor
                        picture: Utils.safeStringOrDefault(
                                     messageData.authorProfilePicture, "")
                        color: CmnCfg.palette.white
                        labelComponent: Ent.ContactLabel {
                            displayName: messageData.authorName
                            username: messageData.author
                            labelColor: CmnCfg.palette.black
                        }
                    }
                }
            }

            Label {
                id: timeInfo
                text: outbound ? qsTr("Sent at: ") + Utils.userTime(
                                     messageData.insertionTime) : qsTr(
                                     "Sent at: ") + Utils.userTime(
                                     messageData.serverTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.Medium
                color: CmnCfg.palette.black
                font.pixelSize: CmnCfg.chatTextSize
            }

            Label {
                id: receiveInfo
                text: qsTr("Received at: ") + Utils.userTime(
                          messageData.insertionTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.Medium
                color: CmnCfg.palette.black
                visible: !outbound
                font.pixelSize: CmnCfg.chatTextSize
            }

            Column {
                Label {
                    text: qsTr("To: ")
                    font {
                        weight: Font.Medium
                        family: CmnCfg.chatFont.name
                        pixelSize: CmnCfg.chatTextSize
                    }
                }

                ListView {
                    width: parent.width
                    model: members
                    height: contentHeight
                    interactive: false

                    delegate: Item {
                        property var memberData: model
                        width: moreInfoPopup.width * 0.75
                        height: visible ? CmnCfg.convoHeight : 0
                        visible: members.rowCount(
                                     ) > 1 ? memberData.userId !== messageData.author : true
                        Common.PlatonicRectangle {
                            boxTitle: memberData.name
                            boxColor: memberData.memberColor
                            picture: Utils.safeStringOrDefault(
                                         memberData.profilePicture, "")
                            property MouseArea hoverHandler
                            color: CmnCfg.palette.white
                            labelComponent: Ent.ContactLabel {
                                displayName: memberData.name
                                username: memberData.userId
                                labelColor: CmnCfg.palette.black
                            }

                            Button {
                                id: receipt
                                anchors.right: parent.right
                                icon.source: Utils.receiptCodeSwitch(
                                                 receiptData[memberData.userId])
                                icon.height: CmnCfg.units.dp(16)
                                icon.width: CmnCfg.units.dp(16)
                                icon.color: CmnCfg.palette.darkGrey
                                padding: 0
                                anchors.verticalCenter: parent.verticalCenter
                                background: Item {}
                            }
                        }
                    }
                }
            }

            ToolButton {
                anchors.horizontalCenter: parent.horizontalCenter
                contentItem: Text {
                    text: qsTr("DELETE MESSAGE")
                    color: CmnCfg.palette.white
                    font.pixelSize: 0
                    font.family: CmnCfg.chatFont.name
                }

                background: Rectangle {
                    color: CmnCfg.palette.alertColor
                }
                onClicked: {
                    mainView.pop()
                    ownedMessages.deleteMessage(ownedMessages.indexById(
                                                    messageData.msgId))
                }
            }
        }
    }
}
