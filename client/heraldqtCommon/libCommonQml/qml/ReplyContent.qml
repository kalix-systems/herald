import QtQuick 2.13

Rectangle {

    color: "green"
    width: metrics.width
    height: 30
    TextMetrics {
        id: metrics
        text: "gogol"
    }
    TextEdit {
        text: "gogol"
    }
}
