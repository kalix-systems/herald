import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

ColumnLayout {
    property string body: ""
    property int epochtimestamp_ms: 100
    property int receiptCode: 0
    property string imageSource: ""
    spacing: 0

    Label {
        id: sender
        Layout.margins: 5
        Layout.preferredHeight: text ? text.height : 0
    }

    Image {
        id: image
        property real aspectRatio: sourceSize.height / sourceSize.height

        Layout.maximumWidth: 500
        Layout.minimumWidth: 200

        Layout.maximumHeight: 500
        //        Layout.minimumHeight: aspectRatio * width
        source: imageSource
        fillMode: sourceSize.height > 500 ? Image.PreserveAspectCrop : Image.PreserveAspectFit
        asynchronous: true
    }

    TextEdit {
        text: body
        Layout.leftMargin: 5
        Layout.rightMargin: 5
        Layout.maximumWidth: image.width - 10 // margin
        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    }

    RowLayout {
        Layout.margins: 5

        Label {
            text: "100"
            id: timestamp
        }

        Item {
            Layout.fillWidth: true
        }

        Image {
            id: receipt
            source: "file:../../icons/double-check-receipt-icon.svg"
            sourceSize: Qt.size(16, 16)
        }
    }
}
