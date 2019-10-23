import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"

RowLayout {
    anchors.fill: parent
    anchors.margins: CmnCfg.units.dp(12)
    spacing: CmnCfg.units.dp(12)

    IconButton {
        id: backButton
        Layout.alignment: Qt.AlignLeft
        tapCallback: searchBarTr
        imageSource: "qrc:/back-arrow-icon.svg"
    }

    IconButton {
        id: clearButton
        Layout.alignment: Qt.AlignRight
        tapCallback: searchBarTr
        imageSource: "qrc:/x-icon.svg"
    }

    function searchBarTr() {
        const cvs = contactViewMain.state
        contactViewMain.state = cvs === "search" ? "default" : "search"
    }
}
