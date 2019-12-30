import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

GridView {
    id: emojiList
    anchors.fill: parent
    boundsBehavior: Flickable.StopAtBounds
    clip: true
    ScrollBar.vertical: ScrollBar {}
    maximumFlickVelocity: 700
    flickDeceleration: emojiList.height * 10
    cellWidth: listView.width / 10
    cellHeight: cellWidth
    cacheBuffer: 1200
    model: emojiPickerModel
    delegate: EmojiButton {
        baseEmoji: model.emoji
        takesModifier: model.skintone_modifier
    }
}
