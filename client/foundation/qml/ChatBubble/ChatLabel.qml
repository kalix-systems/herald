import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    spacing: CmnCfg.smallMargin / 2
    Label {
        id: authorLabel
        text: authorNameTM.elidedText
        property alias authorNameTM: authorNameTM

        font.family: CmnCfg.chatFont.name
        padding: 0
        font.weight: Font.Bold
        font.pixelSize: 13
        color: authorColor
        TextMetrics {
            id: authorNameTM
            text: authorName
            font.weight: Font.Bold
            font.family: CmnCfg.chatFont.name
            elideWidth: bubbleRoot.maxWidth
            elide: Text.ElideRight
        }
    }

    Label {
        id: timestamp
        font.pixelSize: 12
        text: friendlyTimestamp
        color: CmnCfg.palette.darkGrey
        font.family: CmnCfg.chatFont.name
        anchors.bottom: authorLabel.bottom
    }

    Button {
        id: clock
        icon.source: expirationTime !== undefined ? "qrc:/countdown-icon-temp.svg" : ""
        icon.height: 16
        icon.width: 16
        icon.color: "grey"
        padding: 0
        anchors.margins: 0
        background: Item {}
        anchors.verticalCenter: timestamp.verticalCenter
    }
}
