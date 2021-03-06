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

    SystemPalette {
        id: systemPalette
        colorGroup: SystemPalette.Active
    }
    property alias settings: settings

    property bool themeIsDark: ((systemPalette.window.r / Qt.red + systemPalette.window.g / Qt.green
                                 + systemPalette.window.b / Qt.blue) / 256) < 0.5

    readonly property alias sysPalette: systemPalette
    /// standard tiny margin
    readonly property real microMargin: units.dp(4)
    /// standard small margin
    readonly property real smallMargin: units.dp(6)
    /// standard margin size
    readonly property real defaultMargin: units.dp(10)
    /// standard large margin size
    readonly property real largeMargin: units.dp(14)
    /// standard very large margin
    readonly property real megaMargin: units.dp(20)

    // TODO shouldn't use spacers
    /// gap used for tool bars, avatar margins, etc
    readonly property real smallSpacer: units.dp(8)
    /// gap used for larger spacings in tool bars.
    readonly property real largeSpacer: units.dp(12)

    // FONTS

    /// standard header size
    readonly property real headerFontSize: units.dp(17)
    /// size of labels
    readonly property real labelFontSize: units.dp(16)
    /// font size for minor text (e.g. timestamps)
    readonly property int minorTextSize: units.dp(13)
    /// standard chat text size
    readonly property real chatTextSize: units.dp(14)
    readonly property real defaultFontSize: units.dp(15)
    /// size for contact/group name labels in lists
    readonly property int entityLabelSize: units.dp(15)
    /// size for contact/group name labels in lists
    readonly property int entitySubLabelSize: units.dp(14)
    /// standard button text size
    readonly property real buttonTextSize: units.dp(15)
    readonly property real typeMargin: units.dp(28)

    readonly property FontLoader chatFont: metaTheme.chatFont
    readonly property FontLoader labelFont: metaTheme.cairo

    // default font for basic UI text
    readonly property font defaultFont: Qt.font({
                                                    "family": chatFont.name,
                                                    "pixelSize": defaultFontSize
                                                })

    readonly property font headerFont: Qt.font({
                                                   "family": labelFont.name,
                                                   "pixelSize": headerFontSize,
                                                   "weight": Font.DemiBold,
                                                   "letterSpacing": 1
                                               })

    readonly property font sectionHeaderFont: Qt.font({
                                                          "family": labelFont.name,
                                                          "weight": Font.DemiBold,
                                                          "pixelSize": labelFontSize
                                                      })

    // STANDARD COMPONENT SIZES

    /// standard toolbar height
    readonly property real toolbarHeight: units.dp(40)

    /// standard avatar size
    readonly property real avatarSize: units.dp(44)
    readonly property int headerAvatarSize: units.dp(24)
    /// standard conversation/contact height
    readonly property int convoHeight: avatarSize * 1.5

    /// width of chat bubble left accent bar
    readonly property int accentBarWidth: 4

    /// height & width of icon buttons
    readonly property real iconSize: units.dp(22)

    /// height of floating action buttons on home screen
    readonly property real fabDiameter: units.dp(56)

    /// height of floating action buttons on home screen
    readonly property real miniFabDiameter: units.dp(40)

    // MISC
    readonly property int attachmentSize: units.dp(150)
    /// standard z values
    readonly property int overlayZ: 10
    readonly property int topZ: 9
    readonly property int middleZ: 5
    readonly property int bottomZ: 1
    readonly property int underlayZ: -1

    /// user settable cfg
    readonly property int theme: 0

    Themes.MetaThemes {
        id: metaTheme
    }
    /// palette :
    readonly property QtObject palette: metaTheme.themes[theme]
    readonly property var avatarColors: palette.avatarColors

    /// list of recent emojis
    property var recentEmojis: []
    /// fitzpatrick emoji swatch codes
    readonly property var skinSwatchList: ["", "🏻", "🏼", "🏽", "🏾", "🏿"]
    /// emoji skin color
    property int skinSwatchIndex: 0

    Settings {
        id: settings
        readonly property alias skinSwatchIndex: cfg.skinSwatchIndex
        property string recentEmojisJson: "[]"

        Component.onCompleted: {
            recentEmojis = JSON.parse(recentEmojisJson)
        }
    }
}
