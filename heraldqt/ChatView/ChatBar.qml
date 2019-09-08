import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils

ToolBar {
    property alias chatBarAvatar: chatBarAvatar
    property var currentAvatar : Utils.unwrap_or(sideBar.contactsListView.currentItem,
                                                  {contactAvatar: undefined}).contactAvatar


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
        pfpUrl: Utils.unwrap_or(currentAvatar,{pfpUrl: ""}).pfpUrl
        displayName: Utils.unwrap_or(currentAvatar,{displayName: ""}).displayName
        colorHash: Utils.unwrap_or(currentAvatar,{colorHash: 0}).colorHash
    }

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        anchors.fill: parent
    }
}
