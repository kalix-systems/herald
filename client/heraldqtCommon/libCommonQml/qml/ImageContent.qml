import QtQuick 2.13
import QtQuick.Layouts 1.12

Image {
    // ToDo : put the minimum image heights and widths into
    // CFG variables. so they can be made device independant
    height: 0
    width: 250
    onStatusChanged: {
        if (status == Image.Ready) {
            height = Math.min(sourceSize.height, 400)
            width = Math.min(sourceSize.width, 250)
            height = paintedHeight
            width = paintedWidth
        }
    }
    fillMode: Image.PreserveAspectFit
    asynchronous: true
}
