import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../../Common"
import "../../ConfigMenu"
import "../js/CVViewUtils.js" as CVJS

ToolBar {
    id: conversationViewHeader

    clip: true
    height: CmnCfg.toolbarHeight

    background: Rectangle {
        color: CmnCfg.palette.secondaryColor
    }

    RowLayout {
        anchors.fill: parent
        Row {
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: CmnCfg.units.dp(12)
            spacing: CmnCfg.units.dp(16)
            IconButton {
                id: drawerButton
                color: CmnCfg.palette.iconFill
                imageSource: "qrc:/hamburger-icon.svg"
                tapCallback: contextDrawer.open
            }
            Label {
                id: stateLabel
                text: "Messages"
                font {
                    pointSize: CmnCfg.chatPreviewSize
                    family: CmnCfg.chatFont.name
                }
                anchors.verticalCenter: parent.verticalCenter
                color: CmnCfg.palette.iconFill
            }
        }

        Row {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: CmnCfg.units.dp(12)
            spacing: CmnCfg.units.dp(12)

            IconButton {
                id: searchButton
                tapCallback: CVJS.searchBarTr
                color: CmnCfg.palette.iconFill
                imageSource: "qrc:/search-icon.svg"
            }

            IconButton {
                id: configButton
                color: CmnCfg.palette.iconFill
                imageSource: "qrc:/options-icon.svg"
                tapCallback: function () {
                    mainView.push(configMain)
                }
            }
        }
    }
}
