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

        id: chatBarAvatar

        anchors.centerIn: parent
        size: QmlCfg.toolbarHeight - QmlCfg.margin
    }

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        anchors.fill: parent
    }
}
