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
        font.pixelSize: CmnCfg.chatTextSize

        TextMetrics {
            id: opNameTM
            text: (messageModelData.opAuthor
                   === Herald.config.configId) ? Herald.config.name : messageModelData.opName
            font.weight: Font.Bold
            font.family: CmnCfg.chatFont.name
            elideWidth: replyWrapper.width * 0.75
            elide: Text.ElideRight
        }
    }

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {
            replyTs.text = Utils.friendlyTimestamp(
                        messageModelData.opInsertionTime)
        }
    }
    Label {
        id: replyTs

        font.pixelSize: CmnCfg.chatTextSize
        text: Utils.friendlyTimestamp(messageModelData.opInsertionTime)
        color: CmnCfg.palette.darkGrey
        anchors.verticalCenter: replyLabel.verticalCenter
    }
}
