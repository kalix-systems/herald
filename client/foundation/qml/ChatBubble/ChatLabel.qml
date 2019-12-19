import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Layouts 1.12

Label {
    id: authorLabel
    text: authorNameTM.elidedText
    property alias authorNameTM: authorNameTM

    width: parent.width
    anchors.top: parent.top

    padding: CmnCfg.smallMargin / 4
    leftPadding: CmnCfg.smallMargin

    horizontalAlignment: Text.AlignLeft
    font.weight: Font.Bold
    font.family: CmnCfg.chatFont.name

    color: authorColor

    TextMetrics {
        id: authorNameTM
        text: authorName
        font.weight: Font.Bold
        font.family: CmnCfg.chatFont.name
        elideWidth: imageAttach ? 300 : bubbleRoot.maxWidth
        elide: Text.ElideRight
    }
}
