import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Layouts 1.12

Label {
    id: authorLabel
    text: authorNameTM.elidedText
    property alias authorNameTM: authorNameTM

    // width: parent.width

    //anchors.top: parent.top
    horizontalAlignment: Text.AlignLeft
    font.weight: Font.Bold
    font.family: CmnCfg.chatFont.name
    padding: 0

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
