import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3

Window {
    id: imageWindow

    property real scale: 1.0
    property int index: 0
    property var sourceAtc

    readonly property bool sourceValid: sourceAtc !== undefined && index >= 0
    readonly property var reset: function () {//should reset the window
    }

    title: if (imageWindow.sourceValid) {
               sourceAtc[index].path.substring(
                           sourceAtc[index].path.lastIndexOf('/') + 1)
           } else {
               ""
           }

    width: Math.min(image.sourceSize.width, 750)
    height: Math.min(image.sourceSize.height, 500)
    minimumWidth: 350
    minimumHeight: 150

    Action {
        shortcut: StandardKey.MoveToNextChar
        onTriggered: flickable.contentX += flickable.contentWidth * 0.1
    }

    Action {
        shortcut: StandardKey.MoveToPreviousChar
        onTriggered: flickable.contentX -= flickable.contentWidth * 0.1
    }

    Action {
        shortcut: StandardKey.MoveToPreviousLine
        onTriggered: flickable.contentY -= flickable.contentHeight * 0.1
    }

    Action {
        shortcut: StandardKey.MoveToNextLine
        onTriggered: flickable.contentY += flickable.contentHeight * 0.1
    }

    Action {
        id: zoomAction
        shortcut: StandardKey.ZoomIn
        onTriggered: {
            imageWindow.scale += 0.3
            flickable.resizeContent(imageWindow.width * imageWindow.scale,
                                    imageWindow.height * imageWindow.scale,
                                    Qt.point(image.width / 2 + image.x,
                                             image.height / 2 + image.y))
        }
    }

    Action {
        id: zoomOutAction
        shortcut: StandardKey.ZoomOut
        onTriggered: {
            imageWindow.scale -= 0.3
            flickable.resizeContent(imageWindow.width * imageWindow.scale,
                                    imageWindow.height * imageWindow.scale,
                                    Qt.point(image.width / 2 + image.x,
                                             image.height / 2 + image.y))
        }
    }

    Row {
        id: controls
        z: CmnCfg.overlayZ
        spacing: CmnCfg.smallMargin
        padding: CmnCfg.smallMargin
        anchors {
            right: imageWindow.right
            top: imageWindow.top
        }

        Button {
            text: "+"
            font.bold: true
            font.pointSize: 20
            width: 50
            action: zoomAction
        }

        Button {
            text: "―"
            font.bold: true
            font.pointSize: 20
            width: 50
            action: zoomOutAction
        }

        Button {
            text: "↓"
            font.bold: true
            font.pointSize: 20
            width: 50
            onClicked: dirChooser.open()
        }
    }

    FileDialog {
        id: dirChooser
        selectExisting: false
        selectFolder: true
        selectMultiple: false
        folder: StandardPaths.writableLocation(StandardPaths.DesktopLocation)
        onAccepted: herald.utils.saveFile(sourceAtc[index].path, fileUrl)
    }

    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.darkGrey
    }

    onVisibilityChanged: {
        // 2 is the enum for QWindow::Windowed
        // it is not in scope nor in the window namespace
        if (visibility === 2) {
            width = Math.min(image.sourceSize.width, 750)
            height = Math.min(image.sourceSize.height, 500)
            x = root.x
            y = root.y
        }
    }

    Flickable {
        id: flickable
        anchors.fill: parent
        ScrollBar.vertical: ScrollBar {}
        ScrollBar.horizontal: ScrollBar {}
        contentHeight: height
        contentWidth: width
        contentItem.anchors.centerIn: (contentHeight < flickable.height) ? flickable : undefined
        Image {
            id: image
            source: imageWindow.sourceValid ? "file:" + sourceAtc[index].path : ""
            fillMode: Image.PreserveAspectFit
            anchors.fill: parent
            mipmap: true
        }
    }

    PinchArea {
        id: pinchArea
        anchors.fill: parent
        onPinchUpdated: {
            imageWindow.scale += (pinch.scale - pinch.previousScale) * 1.2
            flickable.resizeContent(imageWindow.width * imageWindow.scale,
                                    imageWindow.height * imageWindow.scale,
                                    pinch.center)
        }
    }
}
