import QtQuick 2.14
import QtQuick.Layouts 1.12
import LibHerald 1.0

// NOTE: Here be dragons: this relies on dynamic scoping
// Don't use this outside of the ReplyBubble directory
//wraps doc clip
Item {
    id: fileClip
    property alias nameMetrics: nameMetrics
    property alias fileSize: fileSize

    Layout.preferredHeight: fileIcon.height

    Image {
        id: fileIcon
        anchors.left: parent.left
        anchors.verticalCenter: parent.verticalCenter
        source: "qrc:/file-icon.svg"
        height: 20
        width: height
    }

    TextMetrics {
        id: nameMetrics
        elide: Text.ElideMiddle
        elideWidth: reply.width - fileSize.width - 40 - CmnCfg.smallMargin * 2
    }

    Text {
        id: fileName
        anchors.left: fileIcon.right
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.verticalCenter: parent.verticalCenter
        color: CmnCfg.palette.black
        text: nameMetrics.elidedText
        font.family: CmnCfg.chatFont.name
        font.pixelSize: 13
        font.weight: Font.Medium
    }

    Text {
        id: fileSize
        anchors.left: fileName.right
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.verticalCenter: parent.verticalCenter
        font.family: CmnCfg.chatFont.name
        font.pixelSize: 10
        font.weight: Font.Light
        color: CmnCfg.palette.darkGrey
    }
}
