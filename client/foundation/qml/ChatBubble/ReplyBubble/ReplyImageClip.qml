import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../js/utils.mjs" as Utils
import QtQuick 2.13
import QtGraphicalEffects 1.0

Rectangle {
    property url imageSource
    property int count: 0
    property int clipSize: 64
    width: clipSize
    height: clipSize
    color: "transparent"

    clip: true

    Image {
        id: replyImage
        onSourceChanged: if (source !== undefined) {
                             dims = JSON.parse(Herald.utils.imageScaling(
                                                   Herald.utils.stripUrlPrefix(
                                                       source), clipSize))
                         }

        property var dims
        sourceSize.width: dims === undefined ? 0 : dims.width
        sourceSize.height: dims === undefined ? 0 : dims.height
        anchors.centerIn: parent
        source: parent.imageSource
    }

    ColorOverlay {
        id: overlay
        anchors.fill: parent
        visible: parent.count > 0
        color: CmnCfg.palette.black
        opacity: 0.5
    }

    Text {
        anchors.centerIn: parent
        text: "+ " + parent.count
        visible: overlay.visible
        color: CmnCfg.palette.white
        font.family: CmnCfg.chatFont.name
        font.weight: Font.DemiBold
        font.pointSize: 20
    }
}
