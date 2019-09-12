import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "popups" as Popups
import "../common" as Common
import "../common/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
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
        id: configAvatar
        displayName: Utils.unwrapOr(config.name, config.configId)
        colorHash: config.color
        pfpUrl: Utils.safeStringOrDefault(config.profilePicture, "")
        anchors.horizontalCenter: parent.horizontalCenter
        // JH: Bad margin semantics
        size: parent.height - QmlCfg.margin
    }

    Popups.ConfigPopup {
        id: configPopup
    }

    Common.ButtonForm {
        anchors {
            verticalCenter: parent.verticalCenter
            rightMargin: QmlCfg.margin
            right: parent.right
        }
        source: "qrc:///icons/gear.png"
        onClicked: {
            /// Note: this needs to pay attention to root state
            configPopup.show()
        }
    }
}
