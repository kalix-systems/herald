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
                                    sideBar.contactsListView.currentItem, {
                                        "contactAvatar": undefined
                                    }).contactAvatar
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
    }

    //FC : make a defaultBackground object, just a filling rectange that is mainColor in common
    background: Rectangle {
        color: QmlCfg.palette.mainColor
        anchors.fill: parent
    }
}
