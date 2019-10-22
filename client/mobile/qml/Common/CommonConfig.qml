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
    readonly property real radius: units.largeSpacing
    /// standard margin size used to interior objects
    readonly property real margin: units.largeSpacing
    /// standard half margin
    readonly property real smallMargin: units.smallSpacing
    /// standard half padding unit
    readonly property real smallPadding: units.smallSpacing
    /// standard padding unit
    readonly property real padding: units.smallSpacing
    /// standard toolbar height
    readonly property real toolbarHeight: units.dp(40)
    /// gap used for tool bars, avatar margins, etc
    readonly property real smallSpacer: units.dp(8)
    /// gap used for larger spacings in tool bars.
    readonly property real largeSpacer: units.dp(12)

    /// Font sizes, IMPORTANT: these are in PT, not pixels
    /// standard chat text size
    property int chatTextSize: 14
    /// Chat preview size
    property int chatPreviewSize: 15
    /// standard header size
    property int headerTextSize: 20
    /// standard button text size
    property int buttonTextSize: 17

    /// standard z values
    property int overlayZ: 10
    property int topZ: 9
    property int middleZ: 5
    property int bottomZ: 1
    property int underlayZ: -1
    /// standard avatar size
    property int avatarSize: units.dp(44)
    /// user settable cfg
    property int theme: 0
    ///
    property real avatarHeight: units.dp(48)

    property FontLoader chatFont: metaTheme.chatFont

    property QtObject iconSizes: QtObject {
        property int small: units.dp(16)
        property int smallMedium: units.dp(24)
        property int medium: units.dp(32)
        property int large: units.dp(48)
        property int huge: units.dp(64)
        property int enormous: units.dp(164)
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
