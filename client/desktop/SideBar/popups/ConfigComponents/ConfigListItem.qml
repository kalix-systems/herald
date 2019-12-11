import QtQuick.Controls 2.13
import QtQuick 2.13
import LibHerald 1.0

Column {
    property string headerText
    property Component configContent
    Label {
        text: headerText
        font.family: CmnCfg.labelFont.name
        font.bold: true
        font.pointSize: 24
    }
    Rectangle {
        id: border
        height: 1
        width: parent.width
        color: CmnCfg.palette.offBlack
    }
    Loader {
        sourceComponent: configContent
    }
}
