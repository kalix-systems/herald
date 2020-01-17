pragma Singleton

import QtQuick 2.13
import Qt.labs.settings 1.0
import "qrc:/imports/themes" as Themes
import "qrc:/imports" as Imports

Item {
    id: cfg

    property alias units: importUnits
    property alias settings: settings
    property alias sysPalette: sysPalette
    Imports.Units {
        id: units
    }

    SystemPalette {
        id: sysPalette
        colorGroup: SystemPalette.Active
    }
    /// standard margin size used to interior objects
    readonly property int microMargin: units.dp(4)
    readonly property int smallMargin: units.dp(8)
    readonly property int defaultMargin: units.dp(12)
    readonly property int largeMargin: units.dp(16)
    readonly property int megaMargin: units.dp(24)

    // FONTS
    readonly property FontLoader chatFont: metaTheme.chatFont
    readonly property FontLoader labelFont: metaTheme.cairo

    /// Font size for minor text (e.g. timestamps)
    readonly property int minorTextSize: units.dp(10)
    /// standard chat text size
    readonly property int chatTextSize: units.dp(11)
    /// default font size for basic UI text
    readonly property int defaultFontSize: units.dp(12)
    /// standard header size
    readonly property int labelFontSize: units.dp(14)
    readonly property int headerFontSize: units.dp(15)
    /// size for contact/group name labels in lists
    readonly property int entityLabelSize: units.dp(13)
    /// size for contact/group name labels in lists
    readonly property int entitySubLabelSize: units.dp(11)

    // default font for basic UI text
    readonly property font defaultFont: Qt.font({
                                                    "family": chatFont.name,
                                                    "pixelSize": defaultFontSize
                                                })

    // default font for text in top bar headers
    readonly property font headerFont: Qt.font({
                                                   "family": labelFont.name,
                                                   "weight": Font.DemiBold,
                                                   "letterSpacing": 1,
                                                   "pixelSize": headerFontSize
                                               })

    // default font for text in headings outside the top bar
    readonly property font sectionHeaderFont: Qt.font({
                                                          "family": labelFont.name,
                                                          "weight": Font.DemiBold,
                                                          "pixelSize": headerFontSize
                                                      })

    // STANDARD COMPONENT SIZES

    readonly property int iconSize: units.dp(18)
    /// standard avatar size
    readonly property int avatarSize: units.dp(38)
    readonly property int headerAvatarSize: units.dp(24)
    /// standard conversation/contact height
    readonly property int convoHeight: units.dp(50)
    /// standard toolbar height
    readonly property int toolbarHeight: units.dp(36)
    /// width of chat bubble left accent bar
    readonly property int accentBarWidth: units.dp(4)

    /// standard popup height and width
    readonly property int popupWidth: units.dp(200)
    readonly property int popupHeight: units.dp(250)
    /// standard settings pane width and height
    readonly property int settingsPaneWidth: units.dp(750)
    readonly property int settingsPaneHeight: units.dp(500)

    readonly property real minChatViewWidth: units.dp(300)
    readonly property real minContactsWidth: units.dp(300)
    readonly property real typeMargin: units.dp(20)

    // MISC

    /// standard z values
    readonly property int overlayZ: 10
    readonly property int topZ: 9
    readonly property int middleZ: 5
    readonly property int bottomZ: 1
    readonly property int underlayZ: -1

    readonly property int attachmentSize: 300

    /// list of recent emojis
    property var recentEmojis: []
    /// fitzpatrick emoji swatch codes
    readonly property var skinSwatchList: ["", "🏻", "🏼", "🏽", "🏾", "🏿"]
    /// emoji skin color
    property int skinSwatchIndex: 0

    Imports.Units {
        id: importUnits
    }

    Settings {
        id: settings
        readonly property alias theme: cfg.colorScheme
        readonly property alias skinSwatchIndex: cfg.skinSwatchIndex
        property string recentEmojisJson: "[]"
        Component.onCompleted: {
            recentEmojis = JSON.parse(recentEmojisJson)
        }
    }

    readonly property int colorScheme: 0

    Themes.MetaThemes {
        id: metaTheme
    }
    // palette :
    readonly property QtObject palette: metaTheme.themes[colorScheme]
    readonly property var avatarColors: palette.avatarColors
}
