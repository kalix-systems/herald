import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick 2.14

// TODO this is only used for settings items, arguably not the kind of thing
// we've been calling "labels" elsewhere--make name or usage make more sense
Label {
    font.weight: Font.Medium
    font.family: CmnCfg.labelFont.name
    font.pixelSize: CmnCfg.headerFontSize
    color: CmnCfg.palette.white
}
