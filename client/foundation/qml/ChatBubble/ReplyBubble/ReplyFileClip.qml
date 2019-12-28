import QtQuick 2.14
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import LibHerald 1.0

//wraps doc clip
Row {
    id: fileClip
    property alias nameMetrics: nameMetrics
    property alias fileSize: fileSize
    property real constraint: 0
    property var elideWidth
    height: fileIcon.height
    spacing: CmnCfg.microMargin

    Image {
        id: fileIcon
        source: "qrc:/file-icon.svg"
        height: 20
        width: height
    }

    TextMetrics {
        id: nameMetrics
    }

    RowLayout {
        id: labelWrapper
        anchors.verticalCenter: fileIcon.verticalCenter
        Label {
            id: fileName
            color: CmnCfg.palette.black
            text: nameMetrics.text
            font.family: CmnCfg.chatFont.name
            font.pixelSize: 13
            font.weight: Font.Medium
            maximumLineCount: 1
            elide: Text.ElideMiddle
            Layout.maximumWidth: elideWidth - fileSize.width - fileIcon.width
                                 - constraint - parent.spacing * 3
        }
    }

    Label {
        id: fileSize
        font.family: CmnCfg.chatFont.name
        font.pixelSize: 10
        font.weight: Font.Light
        color: CmnCfg.palette.darkGrey
        anchors.verticalCenter: fileIcon.verticalCenter
    }
}
