import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

ColumnLayout {
    property real maxWidth: Math.min(parent.maxWidth, 600)
    property var mediaParsed
    property var docParsed
    id: wrapperCol

    spacing: 0

    Component.onCompleted: {
        const media = medAttachments.length == 0 ? "" : JSON.parse(
                                                       medAttachments)
        const docs = documentAttachments.length == 0 ? "" : JSON.parse(
                                                           documentAttachments)
        const mediaLen = media.length
        const docLen = docs.length
        mediaParsed = media
        docParsed = docs

        for (var i in docParsed) {
            docModel.append({
                                "path": docParsed[i]
                            })
            print(docParsed)
        }

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

    ListModel {
        id: docModel
    }

    ListView {
        model: docModel
        delegate: Text {
            color: CmnCfg.palette.black
            text: path
            font.family: CmnCfg.chatFontSemiBold.name
            font.pixelSize: 14
        }
    }

    Loader {
        Layout.margins: CmnCfg.smallMargin
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
        }
    }

    Component {
        id: twoImage
        TwoImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
        }
    }

    Component {
        id: threeImage
        ThreeImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
        }
    }

    Component {
        id: fourImage
        FourImageLayout {
            firstImage: mediaParsed[0]
            secondImage: mediaParsed[1]
            thirdImage: mediaParsed[2]
            fourthImage: mediaParsed[3]
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
        }
    }
}
