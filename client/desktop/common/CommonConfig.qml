pragma Singleton

import QtQuick 2.13
import Qt.labs.settings 1.0
import "EmojiJson.js" as JSON
import "qrc:/imports/themes" as Themes
import "qrc:/imports" as Imports

Item {
    id: cfg

    property alias units: importUnits

    Imports.Units {
        id: units
    }
    // TODO do we use this radius anywhere?
    /// edge rounding for all rectangles that use the radius property
    readonly property int radius: 10

    // MARGINS & PADDING

    /// standard margin size used to interior objects
    readonly property int smallMargin: 8
    readonly property int margin: 12
    readonly property int mediumMargin: 16
    readonly property int largeMargin: 20
    /// standard half padding unit
    readonly property int smallPadding: 5
    /// standard padding unit
    readonly property int padding: 10


    // FONTS
    property FontLoader chatFont: metaTheme.chatFont
    property FontLoader labelFont: metaTheme.cairo

    property font headerBarFont: Qt.font({
        family: labelFont.name,
        weight: Font.DemiBold,
        letterSpacing: 1,
        pixelSize: 16
    })

    /// standard chat text size
    property int chatTextSize: 12
    /// standard header size
    property int headerSize: 16


    // STANDARD COMPONENT SIZES

    /// standard avatar size
    property int avatarDiameter: 44
    /// standard conversation/contact height
    property int convoHeight: 56
    /// standard toolbar height
    readonly property int toolbarHeight: 40
    /// standard popup height and width
    property int popupWidth: 200
    property int popupHeight: 250
    /// standard settings pane width and height
    property int settingsPaneWidth: 750
    property int settingsPaneHeight: 400

    property real minChatViewWidth: 300
    property real minContactsWidth: 300


    // MISC

    /// standard z values
    property int overlayZ: 10
    property int topZ: 9
    property int middleZ: 5
    property int bottomZ: 1
    property int underlayZ: -1


    /// fitzpatrick emoji swatch codes
    readonly property var skinSwatchList: ["", "🏻", "🏼", "🏽", "🏾", "🏿"]
    /// emoji skin color
    property int skinSwatchIndex: 0
    /// persistent most common emojis
    readonly property var emojiModel: JSON.emojiJson

    Imports.Units {
        id: importUnits
    }

    SystemPalette {
        id: sysPalette
    }

    Settings {
        id: settings
        property alias theme: cfg.colorScheme
        property alias skinSwatchIndex: cfg.skinSwatchIndex
    }

    property int colorScheme: 0

    Themes.MetaThemes {
        id: metaTheme
    }
    // palette :
    property QtObject palette: metaTheme.themes[colorScheme]
    property var avatarColors: palette.avatarColors
}
