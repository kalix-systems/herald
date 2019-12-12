import QtQuick.Controls 2.13
import QtQuick 2.13
import LibHerald 1.0

RadioButton {
    id: control
    indicator: Rectangle {
        implicitWidth: 20
        implicitHeight: 20
        radius: 20
        border.color: CmnCfg.palette.black
        border.width: 2
        Rectangle {
            radius: 20
            color: CmnCfg.palette.black
            visible: control.checked
            anchors.fill: parent
            anchors.margins: 4
        }
    }
}
