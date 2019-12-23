import QtQuick.Controls 2.13
import Qt.labs.platform 1.1
import QtQuick 2.13
import LibHerald 1.0

// TODO: Make this a menu, perhaps.
// TODO: take a listModel, dumbo.
Rectangle {
    id: self
    property var model: ["no", "model", "defined"]
    property string currentItem: model[0]
    property var onSelected: function () {}

    width: 100
    height: 40

    Menu {
        id: nativeMenu
    }

    // TODO: use instantiator
    Repeater {
        model: self.model
        Item {
            MenuItem {
                id: menuItem
                text: self.model[index]
                onTriggered: {
                    self.currentItem = self.model[index]
                    onSelected()
                }
                Component.onCompleted: {
                    nativeMenu.addItem(menuItem)
                }
            }
        }
    }

    MouseArea {
        anchors.fill: parent
        onClicked: nativeMenu.open()
    }

    Row {
        anchors.fill: parent
        spacing: CmnCfg.defaultMargin
        Label {
            text: currentItem
            anchors.verticalCenter: parent.verticalCenter
            font.family: CmnCfg.labelFont.name
            font.pixelSize: CmnCfg.headerSize
        }

        //TODO: use an SVG image here
        Canvas {
            id: canvas
            anchors.verticalCenter: parent.verticalCenter
            width: 12
            height: 8
            contextType: "2d"
            Component.onCompleted: requestPaint()
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
}
