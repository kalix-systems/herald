import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common"

ToolBar {
    property alias chatBarAvatar: chatBarAvatar

    height: QmlCfg.toolbarHeight
    anchors {
        top: parent.top
        left: parent.left
        right: parent.right
    }

    Avatar {
        id: chatBarAvatar
        anchors.centerIn: parent
    }

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        border.color: QmlCfg.palette.secondaryColor
        anchors.fill: parent
    }
}
