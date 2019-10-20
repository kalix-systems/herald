import QtQuick.Controls 2.0
import QtQuick 2.12
import LibHerald 1.0
import "../Common" as Com

Button {
    property color lbColor: "#5c7598"
    property string lbText: ""

    text: lbText
    font.pixelSize: QmlCfg.mainTextSize

    background: Rectangle {
        id: bg
        color: Qt.darker(lbColor, parent.pressed ? 1.5 : 1.3)
        radius: QmlCfg.radius
        anchors.margins: QmlCfg.smallMargin
    }
}
