import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Label {
    id: authorLabel
    text: authorNameTM.elidedText
    property alias authorNameTM: authorNameTM

    width: parent.width
    anchors.top: parent.top

    padding: CmnCfg.smallMargin / 4
    leftPadding: CmnCfg.smallMargin / 2

    horizontalAlignment: Text.AlignLeft
    font.weight: Font.Bold
    font.family: CmnCfg.chatFont.name
    font.pixelSize: CmnCfg.chatPreviewSize
    color: CmnCfg.palette.white

    background: Rectangle {
        color: authorColor
        border.color: Qt.darker(color, 1.3)
        border.width: 1
    }

    TextMetrics {
        id: authorNameTM
        text: authorName
        font.weight: Font.Bold
        font.pixelSize: CmnCfg.chatPreviewSize
        font.family: CmnCfg.chatFont.name
        elideWidth: imageAttach ? 300 : bubbleRoot.maxWidth
        elide: Text.ElideRight
    }
}
