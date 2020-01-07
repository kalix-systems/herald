import QtQuick.Controls 2.13
import QtQuick 2.13
import LibHerald 1.0

Column {
    property string headerText
    property Component settingsContent
    leftPadding: CmnCfg.defaultMargin
    property alias content: settingsLoader.item
    anchors {
        right: parent.right
        left: parent.left
    }

    Label {
        text: headerText
        font: CmnCfg.sectionHeaderFont
    }

    Loader {
        id: settingsLoader
        anchors {
            right: parent.right
            left: parent.left
        }
        sourceComponent: settingsContent
    }
}
