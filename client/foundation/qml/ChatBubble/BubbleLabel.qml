import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../js/utils.mjs" as Utils

Row {
    spacing: CmnCfg.microMargin
    property alias timestamp: time.text
    property alias name: authorNameTM.text
    Label {
        id: authorLabel
        text: authorNameTM.elidedText
        property alias authorNameTM: authorNameTM

        font.family: CmnCfg.chatFont.name
        padding: 0
        font.weight: Font.Bold
        font.pixelSize: CmnCfg.defaultFontSize
        color: authorColor
        TextMetrics {
            id: authorNameTM
            text: authorName
            font.weight: Font.Bold
            font.family: CmnCfg.chatFont.name
            elideWidth: bubbleRoot.maxWidth - expireInfo.width - timeLabel.width
                        - CmnCfg.smallMargin * 3

            elide: Text.ElideRight
        }
    }

    Label {
        id: timeLabel
        font.pixelSize: CmnCfg.chatTextSize
        text: time.text
        color: CmnCfg.palette.darkGrey
        font.family: CmnCfg.chatFont.name
        anchors.verticalCenter: authorLabel.verticalCenter
        TextMetrics {
            id: time
            font.family: CmnCfg.chatFont.name
            elide: Text.ElideRight
        }

        padding: 0
    }
}
