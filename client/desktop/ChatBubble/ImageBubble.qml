import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string imageSource: ""
    spacing: 0

    ChatLabel {
        id: sender
        senderName: authorName
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

    StandardTextEdit {}

    StandardStamps {}
}
