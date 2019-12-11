import QtQuick.Controls 2.13
import QtQuick 2.13
import LibHerald 1.0

Column {
    property string headerText
    property Component configContent
    leftPadding: CmnCfg.margin
    anchors {
        right: parent.right
        left: parent.left
    }
    Label {
        text: headerText
        font.family: CmnCfg.labelFont.name
        font.bold: true
        font.pointSize: CmnCfg.headerSize
    }
    Loader {
        anchors {
            right: parent.right
            left: parent.left
        }
        sourceComponent: configContent
    }
}
