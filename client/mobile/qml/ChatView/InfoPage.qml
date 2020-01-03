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

    ColumnLayout {
        anchors.fill: parent

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
                    pixelSize: CmnCfg.units.dp(20)
                }
            }
            Row {
                spacing: CmnCfg.defaultMargin
                Av.Avatar {
                    color: CmnCfg.avatarColors[referredChatBubble.messageModelData.authorColor]
                    initials: referredChatBubble.authorName[0].toUpperCase()
                    pfpPath: Utils.safeStringOrDefault(
                                 referredChatBubble.messageModelData.authorProfilePicture)
                    size: CmnCfg.units.dp(80)
                }
                ColumnLayout {
                    height: parent.height
                    spacing: 0
                    Text {
                        padding: 0
                        text: referredChatBubble.messageModelData.authorName
                        font: fromLabel.font
                    }
                    Text {
                        padding: 0
                        text: "@" + referredChatBubble.messageModelData.author
                        font: fromLabel.font
                    }
                }
            }
        }
    }
}
