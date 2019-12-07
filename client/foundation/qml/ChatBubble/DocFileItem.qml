import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

ListView {
    id: docFileItemRoot
    interactive: false
    width: 150
    delegate: Item {
        width: Math.max(bubbleRoot.width, 100)
        height: 24

        Image {
            id: fileIcon
            anchors.verticalCenter: parent.verticalCenter
            source: "qrc:/file-icon.svg"
            height: 20
            width: height
        }

        Text {
            id: fileName
            anchors.verticalCenter: parent.verticalCenter
            anchors.leftMargin: CmnCfg.smallMargin
            anchors.left: fileIcon.right
            color: CmnCfg.palette.black
            text: name
            font.family: CmnCfg.chatFont.name
            font.pixelSize: 13
            elide: Text.ElideRight
            width: {
                const threeMargins = CmnCfg.largeMargin * 3

                if (imageAttach) {
                    return 300 - threeMargins
                } else {
                    return Math.max(docFileItemRoot - 60,
                                    bubbleRoot.messageBody.width - threeMargins)
                }
            }
        }

        Image {
            id: downloadIcon
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            source: "qrc:/download-icon.svg"
            height: 20
            width: height
            anchors.rightMargin: CmnCfg.mediumMargin
        }
    }
}
