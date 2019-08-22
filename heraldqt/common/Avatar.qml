import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12

/// --- displays a list of contacts
Row {
    property string displayName: ""
    ///--- Circle with initial
    leftPadding: 10
    anchors.verticalCenter: parent.verticalCenter
    Rectangle {
        width: rowHeight - 10
        height: rowHeight - 10
        anchors.verticalCenter: parent.verticalCenter
        color:  QmlCfg.palette.mainTextColor
        radius: 100
        ///---- initial
        Text {
            text: qsTr(displayName[0].toUpperCase())
            font.bold: true
            color: QmlCfg.palette.mainColor
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
