import LibHerald 1.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Label {
    readonly property real opNameWidth: opNameTM.width
    text: opNameTM.text
    font.weight: Font.Bold
    font.family: CmnCfg.chatFont.name

    color: opColor
    width: bubbleRoot.width
    horizontalAlignment: Text.AlignLeft

    TextMetrics {
        id: opNameTM
        text: Herald.users.nameById(messageModelData.opAuthor)
    }
}
