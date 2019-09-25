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
    /// GS: this should be bound to global state
    property alias chatBarAvatar: chatBarAvatar
    property var currentAvatar: {
        Utils.unwrapOr(sideBar.conversationsListView.currentItem, {
                           "conversationAvatar": Qt.createComponent(
                                                     "qrc:common/Avatar.qml").createObject(
                                                     parent)
                       }).conversationAvatar
    }
    clip: true
    height: QmlCfg.toolbarHeight
    anchors {
        top: parent.top
        left: parent.left
        right: parent.right
    }

    Common.Avatar {
        id: chatBarAvatar
        anchors.centerIn: parent
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
            onTriggered: convModel.deleteConversation(
                             appRoot.gsCurrentConvo.index)
        }
    }

    Common.ButtonForm {
        id: convOptionsButton
        source: "qrc:/options-icon.svg"
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        onClicked: convOptionsMenu.open()
    }

    Button {
        text: "New member"
        anchors.right: convOptionsButton.left
        onClicked: {
            newMemberPopup.open()
        }
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
                gsConvoItemMembers.addToConversation(userIdText.text,
                                                     convModel.conversationId)
                newMemberPopup.close()
            }
        }
    }

    background: Rectangle {
        color: QmlCfg.avatarColors[chatBarAvatar.colorHash]
        anchors.fill: parent
    }
}
