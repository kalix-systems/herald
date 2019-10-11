pragma Singleton

import QtQuick 2.13
import Qt.labs.settings 1.0
import "EmojiJson.js" as JSON

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

    /// edge rounding for all rectangles that use the radius property
    readonly property int radius: 10
    /// standard margin size used to interior objects
    readonly property int margin: 10
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
    property int popupWidth: 200
    property int configWidth: 600
    property int configHeight: 200

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

    /// palette :
    /// object which contains all of the color configurations
    /// this is defaulted to the Light color scheme
    property var palette: switch (theme) {
        case (0):
        return {
            "mainColor"
            /* mainColor:
            used for backgrounds and default fills*/
            : "white",
            "secondaryColor"
            /* secondaryColor:
             used for secondary lowlighting, borders*/
            : "lightgrey",
            "tertiaryColor"
            /* tertiaryColor:
            used for additional highlighting, selection indication*/
            : "lightsteelblue",
            "tertiaryComplement"
            /* teriaryComplement:
            a direct compliment to the selection color, should be used for alerts
            and uncommon events */
            : "lightsalmon",
            "mainTextColor"
            /* textColor:
             the color of commonly displayed text when it is on
             a surface of the primary color*/
            : "black",
            "secondaryTextColor"
            /* secondaryTextColor:
             the color of text on a surface of the secondaryColor*/
            : "grey",
            "alertTextColor": "red",
            "iconFill": "white",
            "iconMatte": "black",
            "borderColor": "black"
        }
        case (1):
        return {
            "mainColor": "#002b36",
            "secondaryColor": "#073642",
            "tertiaryColor": "#073642",
            "tertiaryComplement": "#839496",
            "mainTextColor": "#839496",
            "secondaryTextColor": "#93a1a1",
            "iconFill": "#eee8d5",
            "alertTextColor": "#dc322f",
            "iconMatte": "#eee8d5",
            "borderColor": "#073642"
        }
    }

    /// Todo : finish these later THIS LIST IS APPEND ONLY
    property var avatarColors: switch (theme) {
        /// none of these besides Light implemented ATM
        case (0):
        return ["#9C2E38", "#ce8054", "#9da86f", "#7498a4", "#bfb35a", "#32a198", "#5e8c6a", "#729eb8", "#CB8C9D"]
        case (1):
        return ["#b58900", "#cb4b16", "#dc322f", "#d33682", "#6c71c4", "#268bd2", "#2aa198", "#859900", "#cd74d4"]
        case (2):
        break
        case (3):
        break
    }
    /// Default Font:
    /// Default Text Size:
    /// Platform :
    /// Global statuses and states :
}
