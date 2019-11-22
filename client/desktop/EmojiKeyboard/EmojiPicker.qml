import QtQuick 2.13
import QtQuick.Controls 2.12
import QtGraphicalEffects 1.12
import LibHerald 1.0

//PAUL: demagic all numbers and colors
Rectangle {
    id: maskShape
    property string modifier: ""
    property var caratCenter
    property var window
    signal send(string emoji)
    signal close

    height: 250
    width: 280
    color: CmnCfg.palette.secondaryColor
    border.color: "#FFFFFF"

    PickerInterior {
        z: 2
        anchors {
            fill: parent
            centerIn: parent
        }
    }
}
