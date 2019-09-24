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
            Layout.alignment: Qt.AlignLeft
            Layout.margins: marginWidth
            Layout.preferredHeight: iconDimLarge
            Layout.preferredWidth: iconDimLarge
            property bool searchRegex: false
            background: Image {
                source: "../heraldqt/icons/Hamburger.png"
                mipmap: true
            }
        }

        // spacer
        Item { Layout.fillWidth: true }

        ToolButton {
            id: searchButton
            Layout.alignment: Qt.AlignRight
            implicitHeight: iconDimLarge
            implicitWidth: iconDimSmall
            property bool searchRegex: false
            background: Image {
                source: "../heraldqt/icons/searchRegexTemp.png"
                mipmap: true
            }
        }


        ToolButton {
            id: toggleButton
            Layout.alignment: Qt.AlignRight
            implicitHeight: iconDimSmall
            implicitWidth: iconDimLarge
            property bool teamView: false
            background: Image {
                source: "../heraldqt/icons/ToggleConv.png"
                mipmap: true
            }
        }

        ToolButton {
            id: menuButton
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: marginWidth
            Layout.leftMargin: -marginWidth
            implicitHeight: iconDimSmall
            implicitWidth: iconDimLarge
            text: qsTr("<b>⋮</b>")
            font.bold: true
            font.pointSize: 24
            background: Rectangle {
                anchors.fill: parent
                color: Qt.darker(secondary, 1.2)
            }
        }

    }
}
