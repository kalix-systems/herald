import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "qrc:/imports" as Imports

ScrollView {
    width: parent.width
    height: wrapperRow.height + 10

    clip: true
    ScrollBar.horizontal.policy: contentWidth > width ? ScrollBar.AlwaysOn : ScrollBar.AlwaysOff
    Row {
        id: wrapperRow
        height: CmnCfg.units.dp(56)
        Layout.margins: CmnCfg.largeMargin
        width: parent.width
        spacing: 5
        Repeater {
            id: imageRepeater
            model: ownedMessages.builder.mediaAttachments
            delegate: Rectangle {
                height: wrapperRow.height
                width: wrapperRow.height
                clip: true
                Image {
                    id: image
                    anchors.fill: parent
                    source: "file:" + mediaAttachmentPath
                    fillMode: Image.PreserveAspectCrop
                    asynchronous: true
                }
            }
        }
    }
}
