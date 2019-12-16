import LibHerald 1.0
import QtQuick 2.14
import QtQuick.Layouts 1.14

Label {
    id: opLabel
    text: knownReply ? Herald.users.nameById(modelData.opAuthor) : ""
    font.bold: true
    Layout.margins: CmnCfg.smallMargin
    Layout.bottomMargin: 0
    Layout.topMargin: CmnCfg.smallMargin
    Layout.preferredHeight: knownReply ? implicitHeight : 0
    color: opColor
}
