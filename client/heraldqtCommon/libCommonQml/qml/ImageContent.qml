import QtQuick 2.13
import QtQuick.Layouts 1.12

Image {
    height: 0
    width: 350
    onStatusChanged: {
        if (status == Image.Ready) {
            height = Math.min(sourceSize.height, 400)
            width = Math.min(sourceSize.width, 350)
            height = paintedHeight
            width = paintedWidth
        }
    }
    fillMode: Image.PreserveAspectFit
    asynchronous: true
}
