import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
ToolBar {
    id: utilityBar
    property int buttonSpacing: 30
    property int marginWidth: 10
    property int iconDimLarge: 25
    property int iconDimSmall: 20
    property int toolBarHeight: 50

    property alias searchButton: searchButton
    property alias drawerButton: drawerButton
    property alias toggleButton: toggleButton
    property alias menuButton: menuButton
    property color secondary: "light gray"

    height: toolBarHeight

    background: Rectangle {
        anchors.fill: parent
        color: Qt.darker(secondary, 1.2)
    }

    RowLayout {
        anchors.fill: parent
        spacing: buttonSpacing
        ToolButton {
            id: drawerButton
            Layout.alignment: Qt.AlignLeft
            Layout.margins: marginWidth
            Layout.preferredHeight: iconDimLarge
            Layout.preferredWidth: iconDimLarge
            property bool searchRegex: false
            background: Image {
                source: "qrc:/hamburger-icon.svg"
                mipmap: true
                sourceSize: Qt.size(iconDimLarge, iconDimLarge)
            }
        }

        // spacer
        Item {
            Layout.fillWidth: true
        }

        ToolButton {
            id: searchButton
            Layout.alignment: Qt.AlignRight
            implicitHeight: iconDimLarge
            implicitWidth: iconDimLarge
            property bool searchRegex: false
            background: Image {
                source: "qrc:/search-icon.svg"
                mipmap: true
                sourceSize: Qt.size(iconDimLarge, iconDimLarge)
            }
        }

        ToolButton {
            id: toggleButton
            Layout.alignment: Qt.AlignRight
            implicitHeight: iconDimSmall
            implicitWidth: iconDimLarge
            property bool teamView: false
            background: Image {
                source: "qrc:/ToggleConv.png"
                sourceSize: Qt.size(iconDimLarge, iconDimLarge)
            }
        }

        ToolButton {
            id: menuButton
            Layout.alignment: Qt.AlignRight
            implicitHeight: iconDimSmall
            implicitWidth: iconDimLarge
            Layout.bottomMargin: marginWidth
            Layout.rightMargin: marginWidth
            font.bold: true
            background: Image {
                source: "qrc:/options-icon.svg"
                height: iconDimLarge
                sourceSize: Qt.size(iconDimLarge, iconDimLarge)
            }
        }
    }
}
