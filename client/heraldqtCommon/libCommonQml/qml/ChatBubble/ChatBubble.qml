import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13

Rectangle {
    id: background
    property color bubbleColor: "light green"
    property string receiptImage: ""
    property string friendlyTimestamp: ""
    property var proxyCfg: {
        radius: 10
    }
    property Component content

    color: bubbleColor
    radius: proxyCfg.radius
    width: contentLoader.width
    height: contentLoader.height

    Loader {
        id: contentLoader
        property int maxWidth: background.parent.width * 0.66
        sourceComponent: content
    }
}
