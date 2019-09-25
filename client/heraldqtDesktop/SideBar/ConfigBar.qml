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
        color: QmlCfg.avatarColors[configAvatar.colorHash]
        // border.color: QmlCfg.palette.secondaryColor
    }

    Common.Avatar {
        id: configAvatar
        avatarLabel: config.displayName
        secondaryText: "@" + config.configId
        colorHash: config.color
        pfpUrl: Utils.safeStringOrDefault(config.profilePicture, "")
        labelGap: 0
        // JH: Bad margin semantics
        anchors.verticalCenter: parent.verticalCenter
        size: parent.height - QmlCfg.margin
        isDefault: false
    }

    Popups.ConfigPopup {
        id: configPopup
    }

    Common.ButtonForm {
        anchors {
            verticalCenter: parent.verticalCenter
            rightMargin: QmlCfg.margin
            right: configButton.left
        }
        source: "qrc:/add-contact-icon.svg"
        onClicked: {
            convoPane.state = "newContactState"
            searchLoader.sourceComponent = searchBarComponent
        }
    }

    Common.ButtonForm {
        id: configButton
        anchors {
            verticalCenter: parent.verticalCenter
            rightMargin: QmlCfg.margin
            right: parent.right
        }
        source: "qrc:/gear-icon.svg"
        onClicked: {
            /// Note: this needs to pay attention to root state
            configPopup.show()
        }
    }
}
