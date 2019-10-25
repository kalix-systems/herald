import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups
import "../common" as Common
import "../../foundation/js/utils.mjs" as Utils
import "../SideBar" as SideBar

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
ToolBar {
    id: contextBar
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.secondaryColor
    }

    RowLayout {
        anchors.fill: parent

        Common.Avatar {
            id: configAvatar
            Layout.margins: CmnCfg.smallMargin
            Layout.alignment: Qt.AlignCenter
            avatarLabel: config.name
            labeled: false
            colorHash: config.color
            pfpUrl: Utils.safeStringOrDefault(config.profilePicture, "")
            labelGap: 0
            size: 28
            isDefault: true
            inLayout: true
        }

        //probably need a standard divider that also handles layouts
        Rectangle {
            Layout.alignment: Qt.AlignHCenter
            height: parent.height
            width: 1
            color: CmnCfg.palette.mainColor
        }

        Text {
            text: "Conversations"
            font.pixelSize: CmnCfg.headerSize
            font.family: CmnCfg.chatFont.name
            font.bold: true
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            color: CmnCfg.palette.mainColor
        }

        Item {
            Layout.fillWidth: true
        }

        Row {
            spacing: 8
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft

            Common.ButtonForm {
                id: searchButton
                property bool searchRegex: false
                fill: CmnCfg.palette.paneColor
                //this is a vertical center offset
                Layout.topMargin: 1
                source: "qrc:/search-icon.svg"
                //todo : add back in regex logic once ui is known
                onClicked: {
                    sideBarState.state = "conversationSearch"
                }
            }

            ///--- Add contact button
            Common.ButtonForm {
                id: newMessageButton
                source: "qrc:/pencil-icon-black.svg"
                fill: CmnCfg.palette.paneColor
                onClicked: {
                    sideBarState.state = "newConversationState"
                }
            }

            //placeholder new contact button
            Common.ButtonForm {
                id: newContactButton
                fill: CmnCfg.palette.paneColor
                source: "qrc:/options-icon.svg"
                onClicked: {
                    contextOptionsMenu.open()
                }
            }
        }

        Popups.ContextOptionsMenu {
            id: contextOptionsMenu
        }
    }
}
