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


    // MARGINS & SPACING

    /// standard tiny margin
    readonly property real microMargin: units.dp(4)
    /// standard small margin
    readonly property real smallMargin: units.dp(8)
    /// standard margin size
    readonly property real defaultMargin: units.dp(12)
    /// standard large margin size
    readonly property real largeMargin: units.dp(16)
    /// standard very large margin
    readonly property real megaMargin: units.dp(24)


    // TODO shouldn't use spacers
    /// gap used for tool bars, avatar margins, etc
    readonly property real smallSpacer: units.dp(8)
    /// gap used for larger spacings in tool bars.
    readonly property real largeSpacer: units.dp(12)


    // FONTS

    /// standard header size
    readonly property real headerTextSize: units.dp(18)
    /// size of labels
    readonly property real labelSize: units.dp(18)
    /// standard chat text size
    readonly property real chatTextSize: units.dp(16)
    /// size for contact/group name labels in lists
    readonly property int entityLabelSize: units.dp(18)
    /// size for contact/group name labels in lists
    readonly property int entitySubLabelSize: units.dp(17)
    /// standard button text size
    readonly property real buttonTextSize: units.dp(17)

    readonly property FontLoader chatFont: metaTheme.chatFont
    readonly property FontLoader labelFont: metaTheme.cairo

    // default font for basic UI text
    readonly property font defaultFont: Qt.font({
        family: chatFont.name,
        pixelSize: units.dp(17)
    })


    // STANDARD COMPONENT SIZES

    /// standard toolbar height
    readonly property real toolbarHeight: units.dp(40)

    /// logged-in user avatar size
    readonly property real identityAvatarDiameter: units.dp(30)
    /// standard avatar size
    readonly property real avatarSize: units.dp(56)

    /// width of chat bubble left accent bar
    readonly property int accentBarWidth: 4

    /// height & width of icon buttons
    readonly property real iconSize: units.dp(24)

    /// height of floating action buttons on home screen
    readonly property real fabDiameter: units.dp(56)

    /// height of floating action buttons on home screen
    readonly property real miniFabDiameter: units.dp(40)


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
