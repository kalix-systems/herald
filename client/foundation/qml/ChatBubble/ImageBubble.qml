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

//    Repeater {
//        model: messageAttachments.mediaAttachments

//        delegate: Image {
//            id: image
//            property real aspectRatio: sourceSize.height / sourceSize.width
//            Layout.maximumWidth: 400
//            Layout.minimumWidth: 200
//            Layout.preferredWidth: sourceSize.width
//            Layout.maximumHeight: 300
//            //TODO: move common typescript into common
//            source: "file:" + mediaAttachmentPath
//            fillMode: Image.PreserveAspectCrop
//            asynchronous: true
//        }
//    }

    RowLayout {
        height: parent.height
        width: parent.width
        Image {
                    id: image
                    property real aspectRatio: sourceSize.height / sourceSize.width
                    width: 100
                     height: width
                    //TODO: move common typescript into common
                    source: "file:" + messageAttachments.mediaAttachments.mediaAttachmentOne
                    fillMode: Image.PreserveAspectCrop
                    asynchronous: true
                }

        Image {
                    id: image2
                    property real aspectRatio: sourceSize.height / sourceSize.width
                   width: 100
                    height: width
                    //TODO: move common typescript into common
                    source: "file:" + messageAttachments.mediaAttachments.mediaAttachmentTwo
                    fillMode: Image.PreserveAspectCrop
                    asynchronous: true
                }

    }

    StandardTextEdit {}

    StandardStamps {}
}
