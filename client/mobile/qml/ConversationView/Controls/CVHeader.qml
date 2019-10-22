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
        anchors.fill: parent
        Row {
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: QmlCfg.units.dp(12)
            spacing: QmlCfg.units.dp(16)
            IconButton {
                id: drawerButton
                imageSource: "qrc:/hamburger-icon.svg"
            }
            Label {
                id: stateLabel
                text: "Messages"
                font {
                    pointSize: QmlCfg.chatPreviewSize
                    family: QmlCfg.chatFont.name
                }
                anchors.verticalCenter: parent.verticalCenter
                color: QmlCfg.palette.iconMatte
            }
        }

        Row {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: QmlCfg.units.dp(12)
            spacing: QmlCfg.units.dp(12)

            IconButton {
                id: searchButton
                tapCallback: CVJS.searchBarTr
                imageSource: "qrc:/search-icon.svg"
            }

            IconButton {
                id: configButton
                imageSource: "qrc:/options-icon.svg"
            }
        }
    }
}
