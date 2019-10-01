import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import Qt.labs.platform 1.1
import "../common" as Common
import "../common/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
ToolBar {
    property var currentAvatar
    clip: true
    height: QmlCfg.toolbarHeight
    // JH: factor z values into the config
    z: 5
    anchors {
        top: parent.top
        left: parent.left
        right: parent.right
    }

    Common.Avatar {
        id: chatBarAvatar
        anchors.left: parent.left
        // JH: Margin fudging
        size: QmlCfg.toolbarHeight - QmlCfg.margin
        pfpUrl: currentAvatar.pfpUrl
        avatarLabel: currentAvatar.avatarLabel
        colorHash: currentAvatar.colorHash
        isDefault: false
    }

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
            visible: isPairwise

            onTriggered: newMemberPopup.open()
        }
    }

    Common.ButtonForm {
        id: convOptionsButton
        source: "qrc:/options-icon.svg"
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        onClicked: convOptionsMenu.open()
    }

    Popup {
        width: 200
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

    background: Rectangle {
        color: QmlCfg.avatarColors[chatBarAvatar.colorHash]
        anchors.fill: parent
    }
}
