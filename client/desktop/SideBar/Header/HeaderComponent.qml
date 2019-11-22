import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "../../SideBar" as SideBar
import "qrc:/imports/Avatar"
import "../popups" as Popups
import QtGraphicalEffects 1.0

//header component loaded during new group & new contact flow
Component {

    ToolBar {
        id: headerBarComponent
        height: CmnCfg.toolbarHeight
        background: Rectangle {
            color: CmnCfg.palette.secondaryColor
        }
        RowLayout {

            anchors.fill: parent

            Common.ConfigAvatar {
            }

            Text {
                id: text
                text: headerLoader.headerText
                font.pixelSize: CmnCfg.headerSize
                font.family: CmnCfg.chatFont.name
                font.bold: true
                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                color: CmnCfg.palette.mainColor
            }
            Item {
                Layout.fillWidth: true
            }

            Common.ButtonForm {
                id: xButton
                fill: CmnCfg.palette.paneColor
                source: "qrc:/x-icon.svg"
                scale: 0.8
                onClicked: sideBarState.state = ""
            }
        }
    }
}
