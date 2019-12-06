import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

ColumnLayout {
    id: wrapperCol
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
    property var mediaParsed
    // callback triggered whenever an image is tapped
    property var imageTappedCallBack: function () {

        print("SHILLLLLENJJK")
    }

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
        mediaParsed = media

        switch (mediaLen) {
        case 0:
            break
        case 1:
            imageLoader.sourceComponent = oneImage
            break
        case 2:
            imageLoader.sourceComponent = twoImage
            break
        case 3:
            imageLoader.sourceComponent = threeImage
            break
        case 4:
            imageLoader.sourceComponent = fourImage
            break
        default:
            imageLoader.sourceComponent = fiveImage
            break
        }
    }

    Loader {
        Layout.margins: CmnCfg.smallMargin
        id: imageLoader
        DropShadow {
            source: parent.item
            anchors.fill: parent.item
            visible: mediaParsed.length > 1
            horizontalOffset: 3
            verticalOffset: 3
            radius: 8.0
            samples: 12
            color: CmnCfg.palette.black
            opacity: 0.55
        }
    }

    Component {
        id: oneImage
        Image {
            property var aspectRatio: mediaParsed[0].width / mediaParsed[0].height
            sourceSize.width: aspectRatio < 1 ? 400 * aspectRatio : maxWidth
            sourceSize.height: aspectRatio < 1 ? 400 : maxWidth / aspectRatio
            source: "file:" + mediaParsed[0].path
            anchors.centerIn: parent
            fillMode: Image.PreserveAspectCrop
            asynchronous: true
            MouseArea {
                onClicked: imageTappedCallBack()
                anchors.fill: parent
            }
        }
    }

    Component {
        id: twoImage
        TwoImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            imageTappedCallback: wrapperCol.imageTappedCallBack()
        }
    }

    Component {
        id: threeImage
        ThreeImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            imageTappedCallback: wrapperCol.imageTappedCallBack()
        }
    }

    Component {
        id: fourImage
        FourImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            fourthImage: mediaParsed[3]
            imageTappedCallback: wrapperCol.imageTappedCallBack()
        }
    }

    Component {
        id: fiveImage
        MultiImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            fourthImage: mediaParsed[3]
            count: mediaParsed.length - 4
            imageTappedCallback: wrapperCol.imageTappedCallBack()
        }
    }

    StandardTextEdit {}

    StandardStamps {}
}
