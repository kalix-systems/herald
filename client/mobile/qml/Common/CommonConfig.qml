import QtQuick 2.13
import Qt.labs.settings 1.0
import "qrc:/imports/themes" as Themes
import "qrc:/imports" as Imports
pragma Singleton

Item {
    id: cfg

    Imports.Units {
        id: units
    }

    readonly property alias units: units

    // TODO shouldn't use rounded corners, get rid of this eventually
    /// edge rounding for all rectangles that use the radius property
    readonly property real radius: units.largeSpacing


    // MARGINS & SPACING

    /// standard small margin
    readonly property real smallMargin: units.dp(8)
    /// standard margin size used to interior objects
    readonly property real defaultMargin: units.largeSpacing


    // TODO shouldn't use spacers
    /// gap used for tool bars, avatar margins, etc
    readonly property real smallSpacer: units.dp(8)
    /// gap used for larger spacings in tool bars.
    readonly property real largeSpacer: units.dp(12)


    // FONTS

    /// size of labels a
    readonly property real labelSize: units.dp(20)
    /// standard chat text size
    readonly property real chatTextSize: units.dp(18)
    /// Chat preview size
    readonly property real chatPreviewSize: units.dp(18)
    /// standard header size
    readonly property real headerTextSize: units.dp(18)
    /// standard button text size
    readonly property real buttonTextSize: units.dp(17)

    readonly property FontLoader chatFont: metaTheme.chatFont
    readonly property FontLoader labelFont: metaTheme.cairo


    // STANDARD COMPONENT SIZES

    /// standard toolbar height
    readonly property real toolbarHeight: units.dp(40)

    /// standard avatar size
    readonly property int avatarSize: units.dp(56)

    /// width of chat bubble left accent bar
    readonly property int accentBarWidth: 4

    // TODO we aren't going to use more than 2-3 sizes for icons, remove this
    // enum once we settle on those sizes
    readonly property QtObject iconSizes: QtObject {
        property int small: units.dp(16)
        property int smallMedium: units.dp(24)
        property int medium: units.dp(32)
        property int large: units.dp(48)
        property int huge: units.dp(64)
        property int enormous: units.dp(164)
    }


    // MISC

    /// standard z values
    readonly property int overlayZ: 10
    readonly property int topZ: 9
    readonly property int middleZ: 5
    readonly property int bottomZ: 1
    readonly property int underlayZ: -1

    /// user settable cfg
    readonly property int theme: 0

    /// emoji skin color
    Settings {
        id: settings
        property alias theme: cfg.theme
    }

    Themes.MetaThemes {
        id: metaTheme
    }
    /// palette :
    readonly property QtObject palette: metaTheme.themes[theme]
    readonly property var avatarColors: palette.avatarColors
}
