import LibHerald 1.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Label {
    id: replyLabel
    readonly property real opNameWidth: opNameTM.width
    text: opNameTM.text
    font.weight: Font.Bold
    font.family: CmnCfg.chatFont.name

    padding: CmnCfg.smallMargin / 4
    color: CmnCfg.palette.white
    leftPadding: CmnCfg.smallMargin / 2
    horizontalAlignment: Text.AlignLeft

    background: Rectangle {
        color: opColor
        border.color: Qt.darker(color, 1.5)
        border.width: 1
    }

    TextMetrics {
        id: opNameTM
        text: Herald.users.nameById(messageModelData.opAuthor)
    }
}
