import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Layouts 1.12

Label {
    id: authorLabel
    text: authorName
    property alias authorNameTM: authorNameTM
    anchors.top: parent.top
    anchors.left: parent.left
    anchors.right: parent.right
    horizontalAlignment: Text.AlignLeft
    font.weight: Font.Bold
    font.family: CmnCfg.chatFont.name
    padding: CmnCfg.smallMargin / 4
    color: CmnCfg.palette.white

    leftPadding: CmnCfg.smallMargin / 2
    background: Rectangle {
        color: authorColor
        border.color: Qt.darker(color, 1.3)
        border.width: 1
    }

    TextMetrics {
        id: authorNameTM
        text: authorName
    }
}
