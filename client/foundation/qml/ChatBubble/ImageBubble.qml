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
    //  property var messageAttachments: null
    property real maxWidth: Math.min(parent.maxWidth, 600)
    property color authorColor
    property bool elided: false
    property bool expanded: false
    property string medAttachments
    property string documentAttachments
    id: wrapperCol

    spacing: 0

    ChatLabel {
        id: uname
        senderName: authorName
        senderColor: authorColor
    }

    Component.onCompleted: {
        wrapperCol.expanded = false
        const media = JSON.parse(medAttachments)
        const docs = JSON.parse(documentAttachments)
        const mediaLen = media.length
        const docLen = docs.length

        switch (mediaLen) {
        case 0:
        case 1:
            const first = media[0]
            const path = first.path
            console.log(path, mediaLen, media[0])
            break
        case 2:

            break
        case 3:
            break
        case 4:
            break
        default:

        }
    }
    GridLayout {

        width: 400
        height: 200

        columns: 2
        rows: 1

        //        Repeater {

        //            model: messageAttachments.mediaAttachments
        //            Layout.fillHeight: true
        //            Layout.fillWidth: true
        //            delegate: Rectangle {
        //                width: 200
        //                height: 200
        //                clip: true
        //                Image {
        //                    id: image
        //                    //TODO: move common typescript into common
        //                    source: messageAttachments.loaded ? "file:" + mediaAttachmentPath : ""
        //                    asynchronous: true
        //                    anchors.centerIn: parent
        //                }
        //            }
        //        }
    }

    //        Rectangle {
    //            property var imageHeight
    //            width: messageAttachments.mediaAttachments.mediaAttachmentWidth(0)
    //            height: messageAttachments.mediaAttachments.mediaAttachmentHeight(0)

    //            Component.onCompleted: print(messageAttachments.mediaAttachments.mediaAttachmentWidth(0))
    //            clip: true

    //            Image {
    //                id: image2
    //                source: messageAttachments.loaded ? "file:" + messageAttachments.mediaAttachments.mediaAttachmentPath(0) : ""
    //                asynchronous: true
    //                anchors.centerIn: parent

    //            }
    //        }
    //    }
    StandardTextEdit {}

    StandardStamps {}
}
