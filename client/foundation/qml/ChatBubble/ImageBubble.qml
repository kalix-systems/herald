import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string authorName: ""
    property Attachments messageAttachments: null
    property var imageTapCallback: function () {
        throw "undefined callback"
    }
    property color authorColor
    property bool elided: false
    property bool expanded: false
    id: wrapperCol
    Component.onCompleted: wrapperCol.expanded = false

    spacing: 0

    ChatLabel {
        id: uname
        senderName: authorName
        senderColor: authorColor
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
            MouseArea {
                anchors.fill: parent
                onClicked: imageTapCallback()
            }
        }
    }

    StandardTextEdit {}

    StandardStamps {}
}
