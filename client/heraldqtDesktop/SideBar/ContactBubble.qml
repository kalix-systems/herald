import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common


Rectangle {
    property alias text: innerText.text
    width: innerText.width + QmlCfg.margin
    height: innerText.height + QmlCfg.margin
    radius: QmlCfg.radius
    border.width: 1
    border.color: "black"
    Text {
        anchors.centerIn: parent
        id: innerText

    }
}
