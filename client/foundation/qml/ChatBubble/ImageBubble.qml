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
    property real maxWidth: Math.min(parent.maxWidth, 600)
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


    RowLayout {
        height: parent.height
        Layout.leftMargin: CmnCfg.smallMargin
        Layout.rightMargin: CmnCfg.smallMargin
        Layout.topMargin: CmnCfg.smallMargin
        Layout.maximumWidth: maxWidth
        clip: true

//        Rectangle {
//            property var imageHeight
//            width: messageAttachments.mediaAttachments.mediaAttachmentWidth(0)
//            height: messageAttachments.mediaAttachments.mediaAttachmentHeight(0)
//            clip: true

//            Image {
//                id: image
//                property real aspectRatio: sourceSize.height / sourceSize.width
//                //TODO: move common typescript into common
//                source: messageAttachments.loaded ? "file:" + messageAttachments.mediaAttachments.mediaAttachmentPath(0) : ""
//                asynchronous: true
//                anchors.centerIn: parent
//            }
//        }


        Rectangle {
            property var imageHeight
            width: 100 //messageAttachments.mediaAttachments.mediaAttachmentWidth(0)
            height: 100 //messageAttachments.mediaAttachments.mediaAttachmentHeight(0)
            clip: true

            Image {
                id: image2
                property real aspectRatio: sourceSize.height / sourceSize.width
                //TODO: move common typescript into common
                source: messageAttachments.loaded ? "file:" + messageAttachments.mediaAttachments.mediaAttachmentPath(0) : ""
                asynchronous: true
                anchors.centerIn: parent

            }
        }
    }

    StandardTextEdit {}

    StandardStamps {}
}
