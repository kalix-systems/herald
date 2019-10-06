import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common


Rectangle {
    id: bubble
    property color defaultColor
    property alias text: innerText.text
    width: innerText.width + QmlCfg.margin
    height: innerText.height + QmlCfg.margin
    color: defaultColor
    radius: QmlCfg.radius
    Text {
        anchors.centerIn: parent
        id: innerText
        color: "white"
        font.bold: true

    }



    states: State {
        name: "clickedstate"
        PropertyChanges {
            target: bubble
            color: Qt.lighter(defaultColor, 1.2)
        }
    }
}
