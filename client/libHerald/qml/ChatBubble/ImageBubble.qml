import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string imageSource: ""
    property string authorName: ""
    property var messageId
    property alias messageAttachments: messageAttachments

    spacing: 0

    ChatLabel {
        id: sender
        senderName: authorName
    }

    Attachments {
        id: messageAttachments
        msgId: messageId
    }

    Repeater {
        model: messageAttachments

   delegate: Image {
        id: image
        property real aspectRatio: sourceSize.height / sourceSize.width
        Layout.maximumWidth: 400
        Layout.minimumWidth: 200
        Layout.preferredWidth: sourceSize.width
        Layout.maximumHeight: 300
        //TODO: move common typescript into common
        source: "file:" + attachmentPath
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
    }
    }

    StandardTextEdit {
    }

    StandardStamps {
    }
}
