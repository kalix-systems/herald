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
    readonly property var skinSwatchList: ["","🏻","🏼","🏽","🏾","🏿"]
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


    /// user settable cfg
    property int theme: 0
    /// emoji skin color
    property int skinSwatchIndex: 0
    /// persistent most common emojis
    readonly property var emojiModel: JSON.emojiJson

    Settings {
        property alias theme: cfg.theme
        property alias skinSwatchIndex: cfg.skinSwatchIndex
    }

    /// palette :
    /// object which contains all of the color configurations
    /// this is defaulted to the Light color scheme
    property var palette: QtObject {
        /// mainColor:
        /// used for backgrounds and default fills
        property string mainColor: "white"
        /// secondaryColor:
        /// used for secondary lowlighting, borders
        property string secondaryColor: "lightgrey"
        /// tertiaryColor:
        /// used for additional highlighting, selection indication
        property string tertiaryColor: "lightsteelblue"
        /// teriaryComplement:
        /// a direct compliment to the selection color, should be used for alerts
        /// and uncommon events
        property string tertiaryComplement: "lightsalmon"
        /// textColor:
        /// the color of commonly displayed text when it is on
        /// a surface of the primary color
        property string mainTextColor: "black"
        /// secondaryTextColor:
        /// the color of text on a surface of the secondaryColor
        property string secondaryTextColor: "grey"
        /// alertTextColor:
        /// Color in alert messages, that can be display on top of the
        /// tertiary color.
        property string alertTextColor: "red"
    }

    Component.onCompleted: {
        switch (theme) {
            /// none of these besides Light implemented ATM
            case (0):
            avatarColors = ["#d93434", "#c48531", "#a68b1e", "#2e8ccf", "#d13a82", "#32a198", "#8ab872", "#729eb8", "#cd74d4"]
            break
            case (1):
            break
            case (2):
            break
            case (3):
            break
        }

    }
    /// Todo : finish these later THIS LIST IS APPEND ONLY
    property var avatarColors: ["#d93434", "#c48531", "#a68b1e", "#2e8ccf", "#d13a82", "#32a198", "#8ab872", "#729eb8", "#cd74d4"]

    /// Default Font:
    /// Default Text Size:
    /// Platform :
    /// Global statuses and states :
}
