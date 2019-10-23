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
    height: CmnCfg.toolbarHeight

    anchors {
        left: parent.left
        right: parent.right
    }

    background: Rectangle {
        color: CmnCfg.avatarColors[configAvatar.colorHash]
    }

    RowLayout {
        anchors.fill: parent
        // PAUL 5: move the label out of avatar. put it in common
        Common.Avatar {
            id: configAvatar
            Layout.topMargin: CmnCfg.smallMargin
            Layout.rightMargin: CmnCfg.margin
            Layout.alignment: Qt.AlignVCenter | Qt.AlignTop | Qt.AlignLeft
            avatarLabel: config.name
            secondaryText: "@" + config.configId
            colorHash: config.color
            pfpUrl: Utils.safeStringOrDefault(config.profilePicture, "")
            labelGap: 0
            // JH: Bad margin semantics
            size: parent.height - CmnCfg.margin
            isDefault: false
            inLayout: true
        }

        Item {
            Layout.fillWidth: true
        }

        Common.ButtonForm {
            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
            Layout.leftMargin: CmnCfg.margin
            source: "qrc:/add-contact-icon.svg"
            onClicked: {
                convoPane.state = "newContactState"
            }
        }

        Common.ButtonForm {
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            Layout.leftMargin: CmnCfg.margin
            Layout.rightMargin: CmnCfg.margin
            id: configButton
            source: "qrc:/gear-icon.svg"
            onClicked: {

                /// Note: this needs to pay attention to root state
                //  configPopup.show()
            }
        }
    }
}
