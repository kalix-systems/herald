import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../popups" as Popups
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports
import "../../SideBar" as SideBar
import "qrc:/imports/Avatar"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0

ToolBar {
    id: contextBar
    height: CmnCfg.toolbarHeight

    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    property alias headerText: headerText.text

    RowLayout {

        anchors.fill: parent
        anchors.rightMargin: 8

        Common.ConfigAvatar {}

        Label {
            id: headerText
            text: qsTr("Converzations")
            Layout.fillWidth: true
            font {
                pixelSize: CmnCfg.headerSize
                family: CmnCfg.labelFont.name
                bold: true
            }
            elide: Text.ElideRight
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            color: CmnCfg.palette.white
        }

        Row {
            spacing: 12
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            Imports.ButtonForm {
                id: searchButton
                property bool searchRegex: false
                fill: CmnCfg.palette.lightGrey
                // this is a vertical center offset
                topPadding: 1
                source: "qrc:/search-icon.svg"
                // TODO : add back in regex logic once ui is known
                onClicked: sideBarState.state = "globalSearch"
            }

            Imports.ButtonForm {
                id: newMessageButton
                source: "qrc:/compose-icon-white.svg"
                fill: CmnCfg.palette.lightGrey
                onClicked: convoMenu.open()
            }

            Imports.ButtonForm {
                id: optionsButton
                fill: CmnCfg.palette.lightGrey
                source: "qrc:/options-icon.svg"
                onClicked: contextOptionsMenu.open()
            }
        }
    }
}
