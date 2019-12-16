import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Layouts 1.12

Label {
    id: authorLabel
    text: authorNameTM.elidedText
    property alias authorNameTM: authorNameTM

    anchors {
        top: parent.top
        left: parent.left
        right: parent.right
    }

    padding: CmnCfg.smallMargin / 4
    leftPadding: CmnCfg.smallMargin / 2

    horizontalAlignment: Text.AlignLeft
    font.weight: Font.Bold
    font.family: CmnCfg.chatFont.name

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
        font.family: CmnCfg.chatFont.name
        elideWidth: bubbleRoot.maxWidth
        elide: Text.ElideRight
    }
}
