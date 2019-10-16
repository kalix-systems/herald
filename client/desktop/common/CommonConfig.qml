pragma Singleton

import QtQuick 2.13
import Qt.labs.settings 1.0
import "EmojiJson.js" as JSON
import "qrc:/imports/themes" as Themes
import "qrc:/imports" as Imports

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
Item {
    id: cfg

    Imports.Units {
        id: units
    }
    /// edge rounding for all rectangles that use the radius property
    readonly property int radius: 10
    /// standard margin size used to interior objects
    readonly property int margin: units.largeSpacing
    /// fitzpatrick emoji swatch codes
    readonly property var skinSwatchList: ["", "🏻", "🏼", "🏽", "🏾", "🏿"]
    /// standard half margin
    readonly property int smallMargin: 5
    /// standard half padding unit
    readonly property int smallPadding: 5
    /// standard padding unit
    readonly property int padding: 10
    /// standard toolbar height
    readonly property int toolbarHeight: 40
    /// standard chat text size
    property int chatTextSize: 10
    /// standard popup height and width
    property int popupWidth: 200
    property int popupHeight: 250
    /// standard config width and height
    property int configWidth: 600
    property int configHeight: 200
    /// standard z values
    property int overlayZ: 10
    property int topZ: 9
    property int middleZ: 5
    property int bottomZ: 1
    property int underlayZ: -1
    /// standard avatar size
    property int avatarSize: 45
    /// standard conversation/contact height
    property int convoHeight: 55

    property real minChatViewWidth: 300
    property real minContactsWidth: 300

    /// user settable cfg
    property int theme: 0
    /// emoji skin color
    property int skinSwatchIndex: 0
    /// persistent most common emojis
    readonly property var emojiModel: JSON.emojiJson

    Settings {
        id: settings
        property alias theme: cfg.theme
        property alias skinSwatchIndex: cfg.skinSwatchIndex
    }

    Themes.MetaThemes {
        id: metaTheme
    }
    /// palette :
    property QtObject palette: metaTheme.themes[theme]
    property var avatarColors: palette.avatarColors
}
