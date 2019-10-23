import QtQuick 2.13

QtObject {
    readonly property color mainColor: "#002b36"
    readonly property color secondaryColor: "#073642"
    readonly property color tertiaryColor: "#073642"
    readonly property color tertiaryComplement: "#839496"
    readonly property color mainTextColor: "#839496"
    readonly property color secondaryTextColor: "#93a1a1"
    readonly property color activeTextColor: "light blue"
    readonly property color visitedColor: "purple"
    readonly property color alertTextColor: "red"
    readonly property color iconFill: "#eee8d5"
    readonly property color iconMatte: "#eee8d5"
    readonly property color borderColor: "#073642"

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

    property color failureColor: alertTextColor
    property color pendingColor: "#F67400"
    property color successColor: "#27AE60"

    property color selectionTextColor: mainTextColor
    property color selectionBackgroundColor: tertiaryColor
    property color tooltipTextColor: mainTextColor
    property color tooltipColor: tertiaryColor

//    property font defaultFont: fontMetrics.font
//    property font markdownFont: fontMetrics.font
    property var avatarColors: ["#b58900", "#cb4b16", "#dc322f", "#d33682", "#6c71c4", "#268bd2", "#2aa198", "#859900", "#cd74d4"]
}
