import QtQuick 2.13

QtObject {
    readonly property color mainColor: "white"
    readonly property color secondaryColor: "lightgrey"
    readonly property color tertiaryColor: "lightsteelblue"
    readonly property color tertiaryComplement: "lightsalmon"
    readonly property color mainTextColor: "black"
    readonly property color secondaryTextColor: "grey"
    readonly property color activeTextColor: "light blue"
    readonly property color visitedColor: "purple"
    readonly property color alertTextColor: "red"
    readonly property color iconFill: "white"
    readonly property color iconMatte: "black"
    readonly property color borderColor: "black"

    property color highlightColor: tertiaryColor
    property color highlightedTextColor: mainColor
    property color backgroundColor: mainColor

    property color iconPressedFill: Qt.darker(iconFill, 1.4)
    property color iconPressedMatte: Qt.lighter(iconMatte, 1.4)

    property color textColor: mainTextColor
    property color disabledTextColor: Qt.darker(mainTextColor, 1.3)
    property color linkColor: activeTextColor
    property color visitedLinkColor: visitedColor
    property color negativeTextColor: alertTextColor
    property color pendingColor: "#F67400"
    property color successColor: "#27AE60"

    property color selectionTextColor: mainTextColor
    property color selectionBackgroundColor: tertiaryColor
    property color tooltipTextColor: mainTextColor
    property color tooltipColor: tertiaryColor

    property font defaultFont: fontMetrics.font
    property font markdownFont: fontMetrics.font
    property var avatarColors: ["#9C2E38", "#ce8054", "#9da86f", "#7498a4", "#bfb35a", "#32a198", "#5e8c6a", "#729eb8", "#CB8C9D"]
}
