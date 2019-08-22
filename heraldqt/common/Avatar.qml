import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12

/// --- displays a list of contacts
Row {
    property string displayName: ""
    property int colorHash: 0
    property int shapeEnum: 0 /// { individual, group ... }
    ///--- Circle with initial
    leftPadding: 10
    anchors.verticalCenter: parent.verticalCenter
    Rectangle {
        width: rowHeight - 10
        height: rowHeight - 10
        anchors.verticalCenter: parent.verticalCenter
        color:  QmlCfg.avatarColors[colorHash]
        radius: shapeEnum == 0 ? width : 0
        ///---- initial
        Text {
            text: qsTr(displayName[0].toUpperCase())
            font.bold: true
            color: "white"
            anchors.centerIn: parent
            font.pixelSize: parent.height - 5
        }
    }
    
    Text {
        text: displayName
        font.bold: true
        anchors.verticalCenter: parent.verticalCenter
    }
    spacing: 10
}
