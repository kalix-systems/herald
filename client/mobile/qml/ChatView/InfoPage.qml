import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "qrc:/imports"
import "../Common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/ChatBubble"
import "qrc:/imports" as Imports

Page {
    id: moreInfoPopup
    property ChatBubble referredChatBubble
    // members of the conversation content
    property var members: []
    readonly property string stateName: "info"
    // list of receipt-user objects. set on completion
    readonly property var receiptData: []

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    Flickable {
        anchors.fill: parent
        anchors.bottomMargin: CmnCfg.largeMargin
        contentHeight: pageContent.height

        Column {
            id: pageContent
            width: parent.width
            padding: CmnCfg.defaultMargin
            spacing: CmnCfg.defaultMargin

            DefaultBubble {
                defaultWidth: parent.width
                convContainer: parent
                messageModelData: referredChatBubble.messageModelData
            }

            Column {
                Label {
                    id: fromLabel
                    text: "From : "
                    font {
                        bold: true
                        family: CmnCfg.labelFont.name
                        pixelSize: CmnCfg.units.dp(18)
                    }
                }
                Row {
                    spacing: CmnCfg.smallMargin
                    Av.Avatar {
                        color: CmnCfg.avatarColors[referredChatBubble.messageModelData.authorColor]
                        initials: referredChatBubble.authorName[0].toUpperCase()
                        pfpPath: Utils.safeStringOrDefault(
                                     referredChatBubble.messageModelData.authorProfilePicture)
                    }
                    Column {
                        spacing: CmnCfg.smallMargin
                        Text {
                            text: referredChatBubble.messageModelData.authorName
                            font.bold: true
                        }
                        Text {
                            text: "@" + referredChatBubble.messageModelData.author
                        }
                    }
                }
            }

            Column {
                Label {
                    text: "To : "
                    font {
                        bold: true
                        family: CmnCfg.labelFont.name
                        pixelSize: CmnCfg.units.dp(18)
                    }
                }

                Repeater {
                    width: parent.width
                    model: members
                    Row {
                        spacing: CmnCfg.smallMargin
                        Av.Avatar {
                            color: CmnCfg.avatarColors[authorColor]
                            initials: authorName[0].toUpperCase()
                            pfpPath: Utils.safeStringOrDefault(
                                         authorProfilePicture)
                        }
                        Column {
                            spacing: CmnCfg.smallMargin
                            Text {
                                text: authorName
                                font.bold: true
                            }
                            Text {
                                text: "@" + author
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
                    ownedMessages.deleteMessage(
                                ownedMessages.indexById(
                                    referredChatBubble.messageModelData.msgId))
                }
            }
        }
    }
}
