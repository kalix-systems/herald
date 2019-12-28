import QtQuick 2.13

QtObject {
    //    readonly property color mainColor: "white"
    //    readonly property color paneColor: "#EFEFEF"
    //    readonly property color secondaryColor: "#27292A"
    //    readonly property color tertiaryColor: "lightsteelblue"
    //    readonly property color tertiaryComplement: "lightsalmon"
    //    readonly property color mainTextColor: "black"
    //    readonly property color secondaryTextColor: "#676C6F"
    //    readonly property color activeTextColor: "light blue"
    //    readonly property color visitedColor: "purple"

    readonly property color iconFill: "white"
    readonly property color iconMatte: "black"
    readonly property color borderColor: "black"
    readonly property color black: "#000000"
    readonly property color offBlack: "#393B3C"
    readonly property color darkGrey: "#676C6F"
    readonly property color medGrey: "#D6D6D6"
    readonly property color lightGrey: "#EFEFEF"
    readonly property color white: "#FFFFFF"
    property color highlightColor: "lightsteelblue"
    property color highlightedTextColor: "white"
    property color backgroundColor: "white"
    readonly property color alertColor: "#C9404B"

    property color iconPressedFill: Qt.darker(iconFill, 1.4)
    property color iconPressedMatte: Qt.lighter(iconMatte, 1.4)

    //    property color textColor: "black"
    //    property color disabledTextColor: Qt.darker(mainTextColor, 1.3)
    //    property color linkColor: activeTextColor
    //    property color visitedLinkColor: visitedColor
    //    property color negativeTextColor: alertTextColor
    //    property color pendingColor: "#F67400"
    //    property color successColor: "#27AE60"

    //    property color selectionTextColor: mainTextColor
    //    property color selectionBackgroundColor: tertiaryColor
    //    property color tooltipTextColor: mainTextColor
    //    property color tooltipColor: tertiaryColor

    property var avatarColors: ["#9C2E38", "#ce8054", "#9da86f", "#7498a4", "#bfb35a", "#32a198", "#5e8c6a", "#729eb8", "#CB8C9D"]
}
