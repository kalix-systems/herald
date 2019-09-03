import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils

ToolBar {
    property alias chatBarAvatar: chatBarAvatar
    clip: true
    height: QmlCfg.toolbarHeight
    anchors {
        top: parent.top
        left: parent.left
        right: parent.right
    }


    Common.Avatar {

        Connections {
            target: sideBar.contactData
            onDataChanged :{ //stutter and forced revaluation of all functions
               chatBarAvatar.get = sideBar.contactData
            }
        }

        property var valid_index: sideBar.contactData.hasIndex
        property int currentIndex: sideBar.contactUi.currentIndex
        property var get: sideBar.contactData

        displayName: if(valid_index(currentIndex,0)) get.name(currentIndex)
        colorHash: if(valid_index(currentIndex,0)) get.color(currentIndex)
        pfpUrl: if(valid_index(currentIndex,0)) get.profile_picture(currentIndex)

        id: chatBarAvatar
        anchors.centerIn: parent
        size: QmlCfg.toolbarHeight - QmlCfg.margin
    }

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        anchors.fill: parent
    }
}
