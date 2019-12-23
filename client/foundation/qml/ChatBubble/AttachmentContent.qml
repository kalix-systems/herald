import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.1

Column {
    id: wrapperCol

    property real maxWidth: Math.min(contentRoot.maxWidth, 600)
    property var mediaParsed
    // callback triggered whenever an image is tapped
    // TODO: Rename this it is nonsense
    property var imageClickedCallBack: function (source) {
        let currentIndex = mediaParsed.findIndex(function (object) {
            if (object === undefined || object === null) {
                return false
            }

            return String("file:" + object.path) === String(source)
        })
        galleryLoader.imageAttachments = mediaParsed
        galleryLoader.currentIndex = currentIndex
        galleryLoader.active = true
        galleryLoader.item.open()
    }

    spacing: 0

    Component.onCompleted: {
        if (medAttachments.length === 0) {
            return
        }

        wrapperCol.mediaParsed = JSON.parse(medAttachments)

        switch (wrapperCol.mediaParsed.length) {
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
        id: imageLoader

        DropShadow {
            source: parent.item
            anchors.fill: parent.item
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
            imageClickedCallBack: wrapperCol.imageClickedCallBack
        }
    }

    Component {
        id: twoImage
        TwoImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            imageClickedCallBack: wrapperCol.imageClickedCallBack
        }
    }

    Component {
        id: threeImage
        ThreeImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            imageClickedCallBack: wrapperCol.imageClickedCallBack
        }
    }

    Component {
        id: fourImage
        FourImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            fourthImage: mediaParsed[3]
            imageClickedCallBack: wrapperCol.imageClickedCallBack
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
            imageClickedCallBack: wrapperCol.imageClickedCallBack
        }
    }
}
