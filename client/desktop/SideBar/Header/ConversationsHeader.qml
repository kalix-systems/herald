import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../popups" as Popups
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports
import "../../SideBar" as SideBar
import "qrc:/imports/Entity"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0

ToolBar {
    id: conversationsHeader
    height: CmnCfg.toolbarHeight + 1

    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    HeaderAvatar {
        anchors {
            verticalCenter: parent.verticalCenter
            left: parent.left
            leftMargin: CmnCfg.smallMargin
        }
    }

    Row {
        anchors {
            verticalCenter: parent.verticalCenter
            right: parent.right
            rightMargin: CmnCfg.smallMargin
        }

        spacing: 12
        Imports.ButtonForm {
            id: searchButton
            property bool searchRegex: false
            fill: CmnCfg.palette.lightGrey
            source: "qrc:/search-icon.svg"
            // TODO : add back in regex logic once ui is known
            onClicked: sideBarState.state = "globalSearch"
        }

        Imports.ButtonForm {
            id: newMessageButton
            source: "qrc:/plus-icon.svg"
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
