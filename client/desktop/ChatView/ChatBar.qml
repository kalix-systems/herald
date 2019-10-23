import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import Qt.labs.platform 1.1
import "../common" as Common
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
        color: CmnCfg.avatarColors[conversationItem.color]
    }

    Common.Avatar {
        id: chatBarAvatar
        anchors.left: parent.left
        avatarLabel: conversationItem.title
        size: CmnCfg.toolbarHeight - CmnCfg.margin
        colorHash: conversationItem.color
        isDefault: false
    }

    Common.ButtonForm {
        id: convOptionsButton
        source: "qrc:/options-icon.svg"
        anchors.right: parent.right
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

            MenuItem {
                text: "Add Member"
                visible: !isPairwise
                onTriggered: newMemberPopup.open()
            }
        }
    }

    Popup {
        width: CmnCfg.popupWidth
        height: 150
        id: newMemberPopup
        TextArea {
            id: userIdText
            placeholderText: "Enter user ID"
        }
        Button {
            height: 50
            text: "Submit"
            anchors.bottom: parent.bottom
            anchors.right: parent.right
            onClicked: {
                convoItemMembers.addToConversation(userIdText.text)
                newMemberPopup.close()
            }
        }
    }
}
