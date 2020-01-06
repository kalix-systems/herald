import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "."

ToolBar {
    id: headerRoot
    property Component headerComponent
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    // header is initially empty, flat and colorless
    Loader {
        id: rootLoader
        anchors.fill: parent
        sourceComponent: headerComponent
    }
}
