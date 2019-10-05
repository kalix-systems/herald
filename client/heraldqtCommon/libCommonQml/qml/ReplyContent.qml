import QtQuick 2.13

Rectangle {

    color: "green"
    width: Math.max(metrics.width)
    height: 30
    TextEdit {
        id: metrics
        text: "gogol"
    }
}
