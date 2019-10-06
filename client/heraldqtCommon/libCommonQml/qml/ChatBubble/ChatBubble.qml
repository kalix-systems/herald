import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13

Rectangle {
    id: background
    property color bubbleColor: "light green"
    property var cfgProxy: {
        // default cfg
        margin: 10
        smallMargins: 5
        radius: 10
    }

    property int receiptCode: 0
    property int epochtimestamp_ms: 100
    property Component content

    color: bubbleColor
    radius: cfgProxy
    width: contentLoader.width
    height: contentLoader.height

    Loader {
        id: contentLoader
        property int maxWidth: background.parent.width * 0.66
        sourceComponent: content
    }
}
