import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ListView {
    // this is a massive hack,
    // gridviews don't allow Section headers
    // so we have to employ a listview of gridlayouts
    // but we have no good way of splitting up data,
    // nor manually indexing into the corpus.
    // so i built this array of boundaries to help me
    // iterate over the corpus.
    // qmlfmt also makes it very, very ugly.
    readonly property var emoji_boundaries: [0, emojiPickerModel.smileys_index, emojiPickerModel.nature_index, emojiPickerModel.food_index, emojiPickerModel.locations_index, emojiPickerModel.activities_index, emojiPickerModel.objects_index, emojiPickerModel.symbols_index, emojiPickerModel.flags_index, emojiPickerModel.rowCount(
            )]

    anchors.fill: parent
    boundsBehavior: Flickable.StopAtBounds
    boundsMovement: Flickable.StopAtBounds
    clip: true
    ScrollBar.vertical: ScrollBar {}
    maximumFlickVelocity: 700
    flickDeceleration: emojiList.height * 10
    model: emoji_boundaries.length // the number of sections
    delegate: Column {
        spacing: 1
        width: parent.width
        Label {
            font {
                family: CmnCfg.labelFont.name
                pixelSize: 16
                bold: true
            }
            color: CmnCfg.palette.white
            text: anchorModel.get(index).sectionName
        }

        GridView {
            id: rep
            interactive: false
            readonly property bool recents: index == 0
            model: recents ? CmnCfg.recentEmojis : emoji_boundaries[index + 1]
                             - emoji_boundaries[index]
            width: parent.width
            height: rep.contentHeight
            cellWidth: parent.width / 10
            cellHeight: cellWidth
            property int base: emoji_boundaries[index]
            delegate: EmojiButton {
                baseEmoji: rep.recents ? CmnCfg.recentEmojis[index].emoji : emojiPickerModel.emoji(
                                             rep.base + index)
                takesModifier: rep.recents ? CmnCfg.recentEmojis[index].takesMod : emojiPickerModel.skintone_modifier(
                                                 rep.base + index)
            }
        }
    }
}
