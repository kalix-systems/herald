import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import QtQuick 2.9
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as Utils

Row {
    id: fileClip
    // set in ReplyComponent.loadDocs()
    property alias nameMetrics: nameMetrics.text
    property alias fileSize: fileSize.text
    property real constraint: 0
    height: fileIcon.height
    spacing: CmnCfg.smallMargin / 2

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
            // TODO: nix magic number
            font.pixelSize: 13
            font.weight: Font.Medium
            maximumLineCount: 1
            elide: Text.ElideMiddle
            Layout.maximumWidth: wrapper.width - fileSize.width - fileIcon.width
                                 - parent.spacing * 3
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
