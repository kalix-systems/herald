import LibHerald 1.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import "../../../js/utils.mjs" as Utils

Row {
    spacing: CmnCfg.microMargin

    Label {
        id: replyLabel
        readonly property real opNameWidth: opNameTM.width
        text: opNameTM.elidedText
        font.weight: Font.Bold
        font.family: CmnCfg.chatFont.name
        padding: 0
        color: opColor
        horizontalAlignment: Text.AlignLeft

        TextMetrics {
            id: opNameTM
            text: messageModelData.opName
            font.weight: Font.Bold
            font.family: CmnCfg.chatFont.name
            elideWidth: imageAttach ? 300 : bubbleRoot.maxWidth
            elide: Text.ElideRight
        }
    }

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {
            replyTs.text = Utils.friendlyTimestamp(
                        messageModelData.opInsertionTime)
            clock.icon.source = messageModelData.opExpirationTime
                    !== undefined ? Utils.timerIcon(
                                        messageModelData.opExpirationTime,
                                        messageModelData.opInsertionTime) : ""
        }
    }
    Label {
        id: replyTs

        font.pixelSize: 11
        text: Utils.friendlyTimestamp(messageModelData.opInsertionTime)
        color: CmnCfg.palette.darkGrey
        anchors.verticalCenter: replyLabel.verticalCenter
    }

    Button {
        id: clock
        icon.source: messageModelData.opExpirationTime
                     !== undefined ? Utils.timerIcon(
                                         messageModelData.opExpirationTime,
                                         messageModelData.opInsertionTime) : ""

        icon.height: 16
        icon.width: 16
        icon.color: "grey"
        padding: 0
        background: Item {}
        anchors.verticalCenter: replyTs.verticalCenter
    }
}
