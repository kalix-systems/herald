import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

ColumnLayout {
    id: atcCol
    property real maxWidth: Math.min(parent.maxWidth, 600)
    property var mediaParsed
    // callback triggered whenever an image is tapped
    property var imageTappedCallBack: function (source) {

        let currentIndex = mediaParsed.findIndex(function (object) {
            // can't use triple equality here because it
            // checks for pointer equivalence...
            return ("file:" + object.path) == source
        })
        imageViewerPopup.sourceAtc = mediaParsed
        imageViewerPopup.index = currentIndex
        imageViewerPopup.reset()
        imageViewerPopup.show()
        imageViewerPopup.raise()
    }

    spacing: 0

    Component.onCompleted: {
        const media = medAttachments.length == 0 ? "" : JSON.parse(
                                                       medAttachments)
        const mediaLen = media.length
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
        OneImageLayout {
            firstImage: mediaParsed[0]
            imageTappedCallback: atcCol.imageTappedCallBack
        }
    }

    Component {
        id: twoImage
        TwoImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            imageTappedCallback: atcCol.imageTappedCallBack
        }
    }

    Component {
        id: threeImage
        ThreeImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            imageTappedCallback: atcCol.imageTappedCallBack
        }
    }

    Component {
        id: fourImage
        FourImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            fourthImage: mediaParsed[3]
            imageTappedCallback: atcCol.imageTappedCallBack
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
            imageTappedCallback: atcCol.imageTappedCallBack
        }
    }
}
