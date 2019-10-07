import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13

Rectangle {
    id: background
    property color bubbleColor: "light green"
    property string receiptImage: ""
    property string friendlyTimestamp: ""
    property real maxWidth: 0
    property Component content

    color: bubbleColor
    width: contentLoader.width
    height: contentLoader.height

    Loader {
        id: contentLoader
        property int maxWidth: parent.maxWidth
        sourceComponent: content
    }
}
