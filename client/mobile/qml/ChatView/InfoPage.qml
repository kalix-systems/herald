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
    readonly property string stateName: "info"
    // list of receipt-user objects. set on completion
    readonly property var receiptData: []

    background: Rectangle {
        color: CmnCfg.palette.white
    }
    Flickable {
        anchors.fill: parent
        contentHeight: pageContent.height
        ColumnLayout {
            id: pageContent
            width: parent.width
            DefaultBubble {
                Layout.fillWidth: true
                defaultWidth: parent.width
                convContainer: parent
                messageModelData: referredChatBubble.messageModelData
                height: referredChatBubble.height
                Layout.alignment: Qt.AlignTop
            }

            Column {
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true
                Layout.leftMargin: CmnCfg.defaultMargin
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

            Row {}

            Row {}

            Column {
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true
                Layout.leftMargin: CmnCfg.defaultMargin
                spacing: CmnCfg.defaultMargin
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
                    model: 8
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
        }
    }
}
