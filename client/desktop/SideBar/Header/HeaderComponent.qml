import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports
import "../../SideBar" as SideBar
import "qrc:/imports/Avatar"
import "../popups" as Popups
import QtGraphicalEffects 1.0

//header component loaded during new group & new contact flow
Component {
    ToolBar {
        id: headerBarComponent
        height: CmnCfg.toolbarHeight + 1
        background: Rectangle {
            color: CmnCfg.palette.offBlack
        }

        Row {
            anchors {
                left: parent.left
                leftMargin: CmnCfg.smallMargin
                 verticalCenter: parent.verticalCenter
            }
            spacing: CmnCfg.margin

            HeaderAvatar {
                anchors.verticalCenter: parent.verticalCenter
            }

            Text {
                id: text
                text: headerLoader.headerText
                font: CmnCfg.headerBarFont
                color: CmnCfg.palette.white
                anchors.verticalCenter: parent.verticalCenter
                // top padding aligns headerText baseline with baseline of
                // initial in user avatar to right
                topPadding: 1
            }
        }


        Imports.ButtonForm {
            anchors {
                verticalCenter: parent.verticalCenter
                right: parent.right
                rightMargin: CmnCfg.smallMargin
            }


            id: xButton
            fill: CmnCfg.palette.lightGrey
            source: "qrc:/x-icon.svg"
            onClicked: {
                sideBarState.state = ""
                Herald.conversationBuilder.clear()
            }
        }
    }
}
