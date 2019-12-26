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


    // MARGINS & SPACING

    /// standard margin size used to interior objects
    readonly property int microMargin: 4
    readonly property int smallMargin: 8
    readonly property int defaultMargin: 12
    readonly property int largeMargin: 16
    readonly property int megaMargin: 24


    // FONTS
    readonly property FontLoader chatFont: metaTheme.chatFont
    readonly property FontLoader labelFont: metaTheme.cairo

    // default font for basic UI text
    readonly property font defaultFont: Qt.font({
        family: chatFont.name,
        pixelSize: 14
    })

    // default font for text in top bar headers
    readonly property font headerBarFont: Qt.font({
        family: labelFont.name,
        weight: Font.DemiBold,
        letterSpacing: 1,
        pixelSize: 16
    })


    /// standard chat text size
    readonly property int chatTextSize: 12
    /// standard header size
    readonly property int headerSize: 16
    /// size for contact/group name labels in lists
    readonly property int entityLabelSize: 14
    /// size for contact/group name labels in lists
    readonly property int entitySubLabelSize: 13


    // STANDARD COMPONENT SIZES

    /// standard avatar size
    readonly property int avatarSize: 44
    /// standard conversation/contact height
    readonly property int convoHeight: 56
    /// standard toolbar height
    readonly property int toolbarHeight: 40
    /// width of chat bubble left accent bar
    readonly property int accentBarWidth: 4


    /// standard popup height and width
    readonly property int popupWidth: 200
    readonly property int popupHeight: 250
    /// standard settings pane width and height
    readonly property int settingsPaneWidth: 750
    readonly property int settingsPaneHeight: 500

    readonly property real minChatViewWidth: 300
    readonly property real minContactsWidth: 300


    // MISC

    /// standard z values
    readonly property int overlayZ: 10
    readonly property int topZ: 9
    readonly property int middleZ: 5
    readonly property int bottomZ: 1
    readonly property int underlayZ: -1


    /// fitzpatrick emoji swatch codes
    readonly property var skinSwatchList: ["", "🏻", "🏼", "🏽", "🏾", "🏿"]
    /// emoji skin color
    readonly property int skinSwatchIndex: 0
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
        readonly property alias theme: cfg.colorScheme
        readonly property alias skinSwatchIndex: cfg.skinSwatchIndex
    }

    readonly property int colorScheme: 0

    Themes.MetaThemes {
        id: metaTheme
    }
    // palette :
    readonly property QtObject palette: metaTheme.themes[colorScheme]
    readonly property var avatarColors: palette.avatarColors
}
