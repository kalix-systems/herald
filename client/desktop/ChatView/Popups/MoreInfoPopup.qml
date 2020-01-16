import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports"
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Ent
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports" as Imports

Popup {
    id: moreInfoPopup
    property var convoMembers: parent.convoMembers
    property var messageData: parent.messageData
    property var ownedMessages: parent.ownedMessages
    property var receiptData
    property var outbound: messageData.author === Herald.config.configId
    modal: true

    height: chatView.height
    width: parent.width
    anchors.centerIn: parent
    onClosed: messageInfoLoader.active = false
    padding: 0
    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }
    onMessageDataChanged: {
        if (messageData === null) {
            moreInfoPopup.close()
        }
    }

    Component.onCompleted: {
        receiptData = JSON.parse(moreInfoPopup.messageData.userReceipts)
    }

    Imports.IconButton {
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.verticalCenter: header.verticalCenter
        icon.source: "qrc:/x-icon.svg"
        fill: CmnCfg.palette.white
        onClicked: {
            moreInfoPopup.close()
            messageInfoLoader.active = false
        }
        z: header.z + 1
    }

    Rectangle {
        id: header
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: CmnCfg.toolbarHeight + 1
        color: CmnCfg.palette.offBlack
        Label {
            id: headerLabel
            anchors.left: parent.left
            anchors.leftMargin: CmnCfg.smallMargin
            text: "Message info"
            font.pixelSize: CmnCfg.headerFontSize
            color: CmnCfg.palette.white
            anchors.verticalCenter: parent.verticalCenter
            font.family: CmnCfg.labelFont.name
        }
    }
    Flickable {
        width: parent.width
        anchors.top: header.bottom
        anchors.bottom: parent.bottom
        contentWidth: width
        contentHeight: wrapperCol.height
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        maximumFlickVelocity: 1500
        flickDeceleration: height * 10
        Column {
            id: wrapperCol
            width: parent.width - CmnCfg.smallMargin * 2
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: CmnCfg.smallMargin
            topPadding: CmnCfg.smallMargin
            bottomPadding: CmnCfg.smallMargin

            Loader {
                id: bubbleInfo
                sourceComponent: messageData.auxData.length === 0 ? bubbleMsg : bubbleAux
                width: parent.width
                height: item.height

                Component {
                    id: bubbleMsg
                    CB.DefaultBubble {
                        convContainer: parent
                        defaultWidth: parent.width
                        width: parent.width
                        messageModelData: moreInfoPopup.messageData
                    }
                }

                Component {
                    id: bubbleAux
                    CB.AuxBubble {
                        defaultWidth: parent.width
                        width: parent.width
                        messageModelData: moreInfoPopup.messageData
                        auxData: JSON.parse(messageModelData.auxData)
                        moreInfo: true
                    }
                }
            }
            Label {
                id: senderHeader
                anchors.left: bubbleInfo.left
                text: "From:"
                font.family: CmnCfg.chatFont.name
                font.weight: Font.Medium
                color: CmnCfg.palette.black
                font.pixelSize: CmnCfg.chatTextSize
            }

            Item {
                id: author
                anchors.left: senderHeader.left
                height: CmnCfg.convoHeight
                width: parent.width
                Common.PlatonicRectangle {

                    topTextMargin: CmnCfg.smallMargin
                    bottomTextMargin: CmnCfg.defaultMargin
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
                    MouseArea {
                        id: hoverHandler
                    }
                }
            }

            Label {
                id: timeInfo
                anchors.left: author.left
                text: outbound ? "Sent at: " + Utils.userTime(
                                     messageData.insertionTime) : "Sent at: " + Utils.userTime(
                                     messageData.serverTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.Medium
                color: CmnCfg.palette.black
                font.pixelSize: CmnCfg.chatTextSize
            }

            Label {
                id: receiveInfo
                anchors.left: author.left
                text: "Received at: " + Utils.userTime(
                          messageData.insertionTime)
                font.family: CmnCfg.chatFont.name
                font.weight: Font.Medium
                color: CmnCfg.palette.offBlack
                visible: !outbound
                font.pixelSize: CmnCfg.chatTextSize
            }

            Label {
                id: expireInfo
                anchors.left: timeInfo.left
                visible: messageData.expirationTime !== undefined
                text: messageData.expirationTime
                      !== undefined ? "Expires at: " + Utils.userTime(
                                          messageData.expirationTime) : ""
                font.family: CmnCfg.chatFont.name
                font.weight: Font.Medium
                color: CmnCfg.palette.black
                font.pixelSize: CmnCfg.chatTextSize
            }

            Label {
                id: recHeader
                anchors.left: timeInfo.left
                text: "To:"
                font.family: CmnCfg.chatFont.name
                font.weight: Font.Medium
                color: CmnCfg.palette.black
                font.pixelSize: CmnCfg.chatTextSize
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
                    width: 300
                    // TODO special-case Note to Self conversations so they
                    // don't have an empty recipient list
                    visible: convoMembers.rowCount(
                                 ) > 1 ? memberData.userId !== messageData.author : true
                    property var memberData: model
                    Common.PlatonicRectangle {
                        boxTitle: memberData.name
                        boxColor: memberData.memberColor
                        picture: Utils.safeStringOrDefault(
                                     memberData.profilePicture, "")
                        property MouseArea hoverHandler
                        topTextMargin: CmnCfg.smallMargin
                        bottomTextMargin: CmnCfg.defaultMargin
                        color: CmnCfg.palette.white
                        labelComponent: Ent.ContactLabel {
                            displayName: memberData.name
                            username: memberData.userId
                            labelColor: CmnCfg.palette.black
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

                            anchors.bottom: parent.bottom
                            anchors.bottomMargin: CmnCfg.smallMargin
                            background: Item {}
                        }

                        states: []
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
                    deleteMsgPrompt.deleteId = messageData.msgId
                    deleteMsgPrompt.open()
                }
            }
        }
    }

    MessageDialog {
        id: deleteMsgPrompt
        property var deleteId
        text: qsTr("Delete message")
        informativeText: qsTr("Do you want to delete this message from this device?")
        standardButtons: MessageDialog.Ok | MessageDialog.Cancel

        onAccepted: {
            // prevent coercion of undefined into bytearray
            if (deleteId === undefined) {
                return
            }

            moreInfoPopup.close()
            messageInfoLoader.active = false

            ownedMessages.deleteMessageById(deleteId)
            deleteId = undefined
        }
    }
}
