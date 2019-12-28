import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"

RowLayout {
    anchors.fill: parent
    anchors.margins: CmnCfg.units.dp(12)
    spacing: CmnCfg.units.dp(12)

    AnimIconButton {
        id: backButton
        Layout.alignment: Qt.AlignLeft
        tapCallback: searchBarTr
        imageSource: "qrc:/back-arrow-icon.svg"
    }

    AnimIconButton {
        id: clearButton
        Layout.alignment: Qt.AlignRight
        tapCallback: searchBarTr
        imageSource: "qrc:/x-icon.svg"
    }

    function searchBarTr() {
        if (contactViewMain.state === "search") {
            contactViewMain.state = "default"
        } else {
            contactViewMain.state = "search"
        }
    }
}
