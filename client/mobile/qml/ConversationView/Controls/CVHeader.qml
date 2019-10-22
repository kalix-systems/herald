import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../../Common"
import "../js/CVViewUtils.js" as CVJS

ToolBar {
    id: conversationViewHeader

    clip: true
    height: QmlCfg.toolbarHeight

    background: Rectangle {
        color: QmlCfg.palette.secondaryColor
    }

    RowLayout {
        anchors {
            fill: parent
            rightMargin: QmlCfg.margin
            leftMargin: QmlCfg.margin
        }
        IconButton {
            id: drawerButton
            Layout.alignment: Qt.AlignLeft
            imageSource: "qrc:/hamburger-icon.svg"
        }

        Label {
            id: stateLabel
            text: "Messages"
            font.pointSize: QmlCfg.headerTextSize
            Layout.alignment: Qt.AlignHCenter
            color: QmlCfg.palette.iconMatte
        }

        Row {
            Layout.alignment: Qt.AlignRight
            spacing: QmlCfg.units.dp(12)

            IconButton {
                id: searchButton
                Layout.alignment: Qt.AlignRight
                tapCallback: CVJS.searchBarTr
                imageSource: "qrc:/search-icon.svg"
            }

            IconButton {
                id: configButton
                Layout.alignment: Qt.AlignRight
                imageSource: "qrc:/options-icon.svg"
            }
        }
    }
}
