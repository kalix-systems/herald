import QtQuick 2.13
import Qt.labs.settings 1.0
import "qrc:/imports/themes" as Themes
import "qrc:/imports" as Imports
pragma Singleton

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

    readonly property alias units: units
    /// edge rounding for all rectangles that use the radius property
    readonly property int radius: units.largeSpacing
    /// standard margin size used to interior objects
    readonly property int margin: units.largeSpacing
    /// standard half margin
    readonly property int smallMargin: units.smallSpacing
    /// standard half padding unit
    readonly property int smallPadding: units.smallSpacing
    /// standard padding unit
    readonly property int padding: units.smallSpacing
    /// standard toolbar height
    readonly property int toolbarHeight: units.dp(40)
    /// standard chat text size
    property int chatTextSize: units.dp(12)
    /// standard header size
    property int headerSize: units.gu(3)
    /// standard button text size
    property int buttonTextSize: units.dp(16)

    /// standard z values
    property int overlayZ: 10
    property int topZ: 9
    property int middleZ: 5
    property int bottomZ: 1
    property int underlayZ: -1
    /// standard avatar size
    property int avatarSize: iconSizes.large
    /// user settable cfg
    property int theme: 0
    readonly property string mainTextFontFamily: "Hevletica"

    property QtObject iconSizes: QtObject {
        property int small: Math.floor(units.fontMetrics.roundedIconSize(
                                           16 * units.devicePixelRatio))
        property int smallMedium: Math.floor(units.fontMetrics.roundedIconSize(
                                                 22 * units.devicePixelRatio))
        property int medium: Math.floor(units.fontMetrics.roundedIconSize(
                                            32 * units.devicePixelRatio))
        property int large: Math.floor(units.fontMetrics.roundedIconSize(
                                           48 * units.devicePixelRatio))
        property int huge: Math.floor(units.fontMetrics.roundedIconSize(
                                          64 * units.devicePixelRatio))
        property int enormous: Math.floor(128 * units.devicePixelRatio)
    }

    /// emoji skin color
    Settings {
        id: settings
        property alias theme: cfg.theme
    }

    Themes.MetaThemes {
        id: metaTheme
    }
    /// palette :
    property QtObject palette: metaTheme.themes[theme]
    property var avatarColors: palette.avatarColors
}
