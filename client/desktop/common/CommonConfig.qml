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
    // TODO do we use this radius anywhere?
    /// edge rounding for all rectangles that use the radius property
    readonly property int radius: 10
    SystemPalette {
        id: sysPalette
        colorGroup: SystemPalette.Active
    }
    /// standard margin size used to interior objects
    readonly property int microMargin: 4
    readonly property int smallMargin: 8
    readonly property int defaultMargin: 12
    readonly property int largeMargin: 16
    readonly property int megaMargin: 24

    // FONTS
    readonly property FontLoader chatFont: metaTheme.chatFont
    readonly property FontLoader labelFont: metaTheme.cairo

    /// Font size for minor text (e.g. timestamps)
    readonly property int minorTextSize: 11
    /// standard chat text size
    readonly property int chatTextSize: 12
    /// default font size for basic UI text
    readonly property int defaultFontSize: 14
    /// standard header size
    readonly property int headerFontSize: 16
    /// size for contact/group name labels in lists
    readonly property int entityLabelSize: 14
    /// size for contact/group name labels in lists
    readonly property int entitySubLabelSize: 13

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

    /// standard avatar size
    readonly property int avatarSize: 44
    readonly property int chatAvatarSize: 36
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
