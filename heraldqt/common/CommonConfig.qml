pragma Singleton
import QtQuick 2.5

Item {
    Theme { id: themeEnum }
    property int theme: themeEnum.light /// user settable
    /// edge rounding for all rectangles
    /// that use the radius property
    readonly property int radius: 10
    /// palette :
    /// object which contains all of the color configurations
    /// this is defaulted to the Light color scheme
    property var palette: QtObject{
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
        property string secondaryTextColor: "black"
        /// alertTextColor:
        /// Color in alert messages, that can be display on top of the
        /// tertiary color.
        property string alertTextColor: "red"
    }

    Component.onCompleted: {
                switch(theme) {
                /// none of these besides Light implemented ATM
                case (themeEnum.light) :
                    avatarColors = ["#d93434","#c48531","#a68b1e",
                                    "#2e8ccf","#d13a82","#32a198",
                                    "#8ab872","#729eb8","#cd74d4"]
                    break;
                case(themeEnum.dark) :
                    break;
                case(themeEnum.solarized_dark) :
                    break;
                case(themeEnum.solarized_light) :
                    break;
                }
    }
    /// Todo : finish these later
    property var avatarColors: ["#d93434","#c48531","#a68b1e",
                                "#2e8ccf","#d13a82","#32a198",
                                "#8ab872","#729eb8","#cd74d4"]

    /// Default Font:
    /// Default Text Size:
    /// Platform :
    /// Global statuses and states :
}
