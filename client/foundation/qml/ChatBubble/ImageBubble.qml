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
    property Attachments messageAttachments: null
     property color authorColor

    spacing: 0


    Row {
        Layout.margins: CmnCfg.smallMargin / 2
        Layout.bottomMargin: 0
        spacing: CmnCfg.smallMargin / 2

        ChatLabel {
            id: uname
            senderName: authorName
            senderColor: authorColor
        }

        Label {
            id: timestamp
            text: friendlyTimestamp
            color: CmnCfg.palette.secondaryTextColor
            font.pixelSize: 10
            anchors {
                top: parent.top
                topMargin: 3
            }
        }
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