import QtQuick.Controls 2.13
import QtQuick 2.13
import LibHerald 1.0

ComboBox {
    id: self
    background: Rectangle {
        implicitWidth: 100
        implicitHeight: 40
    }

    indicator: Canvas {
        id: canvas
        x: self.width - width - self.rightPadding
        y: self.topPadding + (self.availableHeight - height) / 2
        width: 12
        height: 8
        contextType: "2d"
        Component.onCompleted: canvas.requestPaint()
        onPaint: {
            context.reset()
            context.moveTo(0, 0)
            context.lineTo(width, 0)
            context.lineTo(width / 2, height)
            context.closePath()
            context.fill()
        }
    }
}
