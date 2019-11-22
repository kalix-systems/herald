import QtQuick 2.13
import QtQuick.Shapes 1.12

Item {
    // timer assigned by the constructor
    property Timer timer
    // time remaining in seconds
    property int timeRemaining
    // original length of the fuse
    property int originalPeriod
    Rectangle {
        anchors.fill: parent
        color: "green"
    }

    Shape {
        anchors.fill: parent
        PathArc {
            x: parent.width * 0.2
            y: parent.height * 0.2
            radiusX: 100
            radiusY: radiusY
        }
    }
}
