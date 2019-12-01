import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.0
import LibHerald 1.0

Button {
    property string imageSource: ""
    property int anchorIndex

    onClicked: emojiList.contentY = innerRepeater.itemAt(anchorIndex).y
    padding: 0
    background: Item {
    }
    icon.source: imageSource
    icon.color: CmnCfg.palette.white
    icon.width: 15
    icon.height: 15
}
