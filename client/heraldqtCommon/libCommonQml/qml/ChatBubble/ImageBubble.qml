import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

ColumnLayout {
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string imageSource: ""
    spacing: 0

    Label {
        id: sender
        Layout.margins: 5
        Layout.preferredHeight: text ? text.height : 0
    }

    Image {
        id: image
        property real aspectRatio: sourceSize.height / sourceSize.width
        Layout.maximumWidth: 400
        Layout.minimumWidth: 200
        Layout.preferredWidth: sourceSize.width
        Layout.maximumHeight: 300
        source: imageSource
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
    }

    TextEdit {
        text: body
        Layout.leftMargin: 5
        Layout.rightMargin: 5
        Layout.preferredHeight: body.length === 0 ? 0 : undefined
        Layout.maximumWidth: image.width - 10 // margin
        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    }

    RowLayout {
        Layout.margins: 5

        Label {
            text: friendlyTimestamp
            id: timestamp
        }

        Item {
            Layout.fillWidth: true
        }

        Image {
            id: receipt
            source: receiptImage
            sourceSize: Qt.size(16, 16)
        }
    }
}
