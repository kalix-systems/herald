import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
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
    // NPB: wat.
    property var currentAvatar: Utils.unwrapOr(
                                    sideBar.conversationsListView.currentItem, {
                                        "conversationAvatar": undefined
                                    }).conversationAvatar
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
        size: QmlCfg.toolbarHeight - QmlCfg.margin
        // NPB: more wat. this is why unwrap or needs to do more things
        // perhaps write something like map_err here
        pfpUrl: Utils.unwrapOr(currentAvatar, {
                                   "pfpUrl": ""
                               }).pfpUrl
        displayName: Utils.unwrapOr(currentAvatar, {
                                        "displayName": ""
                                    }).displayName
        colorHash: Utils.unwrapOr(currentAvatar, {
                                      "colorHash": 0
                                  }).colorHash
        isDefault: false
    }

    Button {
        text: "New member"
        anchors.right: parent.right
        onClicked: { newMemberPopup.open()
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
                gsConvoItemMembers.addToConversation(userIdText.text, gsConversationId)
                newMemberPopup.close()
            }
        }
    }



    background: Rectangle {
        color: QmlCfg.avatarColors[chatBarAvatar.colorHash]
        anchors.fill: parent
    }
}
