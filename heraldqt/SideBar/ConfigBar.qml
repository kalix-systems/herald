import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "popups" as Popups
import "../common" as Common
import "../common/utils.js" as Utils

ToolBar {
    id: toolBar
    anchors {
        left: parent.left
        right: parent.right
        top: parent.top
    }
    height: QmlCfg.toolbarHeight

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        border.color: QmlCfg.palette.secondaryColor
    }

    Common.Avatar {
        id: contactAvatar
        displayName: Utils.unwrap_or(config.name, config.id)
        colorHash: 0
        pfpUrl: Utils.unwrap_or(config.profile_picture, "")
        anchors.horizontalCenter: parent.horizontalCenter
        size: parent.height - QmlCfg.margin
    }

    /// unpolished temporary Popup
    Popups.ConfigPopup {
        id: configPopup
    }

    Button {
        height: parent.height
        width: height
        anchors.right: parent.right
        background: Image {
            source: "qrc:///icons/gear.png"
            width: parent.height
            height: width
            scale: 0.7
            mipmap: true
        }
        onClicked: {
            configPopup.open()
        }
    }
}
