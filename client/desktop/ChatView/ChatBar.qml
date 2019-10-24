import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import Qt.labs.platform 1.1
import "../common" as Common
import "qrc:/imports/Avatar"
import "../../foundation/js/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
ToolBar {
    property var conversationItem
    property Messages ownedConversation: parent.ownedConversation

    height: CmnCfg.toolbarHeight
    z: CmnCfg.middleZ

    background: Rectangle {
        color: CmnCfg.palette.secondaryColor
    }

    AvatarMain {
        iconColor: CmnCfg.palette.iconFill
        textColor: CmnCfg.avatarColors[conversationItem.color]
        size: CmnCfg.units.dp(36)
        initials: conversationItem.title[0].toUpperCase()
        anchors {
            margins: CmnCfg.units.dp(18)
        }
    }

    Common.ButtonForm {
        id: convOptionsButton
        source: "qrc:/options-icon.svg"
        anchors.right: parent.right
        fill: CmnCfg.palette.paneColor
        anchors.verticalCenter: parent.verticalCenter
        onClicked: convOptionsMenu.open()
        Menu {
            id: convOptionsMenu

            MenuItem {
                text: "Archive"
            }

            MenuItem {
                text: "Clear History"
                onTriggered: ownedConversation.clearConversationHistory()
            }
        }
    }
}
