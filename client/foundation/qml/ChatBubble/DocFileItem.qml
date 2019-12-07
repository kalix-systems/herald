import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

ListView {
    interactive: false
    width: 80
    Layout.maximumWidth: parent.width
    delegate: Item {
        height: 24
        width: bubbleRoot.width
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
            width: imageAttach ? 300 - CmnCfg.largeMargin * 3 : Math.max(
                                     40,
                                     messageBody.width - CmnCfg.largeMargin * 3)
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
