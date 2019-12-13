import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../js/utils.mjs" as Utils
import QtQuick 2.13
import QtGraphicalEffects 1.0

Rectangle {
    property real aspectRatio
    property string imageSource
    property int count
    width: 64
    height: 64

    clip: true
    color: "transparent"
    Image {
        id: replyImage
        sourceSize.width: parent.aspectRatio < 1 ? 64 : 64 * parent.aspectRatio
        sourceSize.height: parent.aspectRatio < 1 ? 64 / parent.aspectRatio : 64
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
