import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0

TextField {
    id: lpTextField

    placeholderText: qsTr("Enter Username") + "..."

    width: CmnCfg.units.gu(15)

    background: Rectangle {
        color: "#FFFFFF"
    }
}
