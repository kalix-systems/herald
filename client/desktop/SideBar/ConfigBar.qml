import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "popups" as Popups
import "../common" as Common
import "../common/js/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
ToolBar {
    id: toolBar
    height: QmlCfg.toolbarHeight

    anchors {
        left: parent.left
        right: parent.right
    }

    background: Rectangle {
        color: QmlCfg.avatarColors[configAvatar.colorHash]
    }

    RowLayout {
        anchors.fill: parent
        // PAUL 5: move the label out of avatar. put it in common
        Common.Avatar {
            id: configAvatar
            Layout.topMargin: QmlCfg.smallMargin
            Layout.rightMargin: QmlCfg.margin
            Layout.alignment: Qt.AlignVCenter | Qt.AlignTop | Qt.AlignLeft
            avatarLabel: config.name
            secondaryText: "@" + config.configId
            colorHash: config.color
            pfpUrl: Utils.safeStringOrDefault(config.profilePicture, "")
            labelGap: 0
            // JH: Bad margin semantics
            size: parent.height - QmlCfg.margin
            isDefault: false
            inLayout: true
        }

        Popups.ConfigPopup {
            id: configPopup
        }

        Common.ButtonForm {
            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
            Layout.leftMargin: QmlCfg.margin
            source: "qrc:/add-contact-icon.svg"
            onClicked: {
                // BNOTE: Is this the right order? It might be, but check
                convoPane.state = "newContactState"
                searchLoader.sourceComponent = searchBarComponent
            }
        }

        Common.ButtonForm {
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            Layout.leftMargin: QmlCfg.margin
            id: configButton
            source: "qrc:/gear-icon.svg"
            onClicked: {
                /// Note: this needs to pay attention to root state
                configPopup.show()
            }
        }
    }
}
