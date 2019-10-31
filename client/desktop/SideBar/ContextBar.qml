import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups
import "../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "../SideBar" as SideBar
import "qrc:/imports/Avatar"
import QtGraphicalEffects 1.0

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
    property alias headerText: headerText.text

    RowLayout {

        anchors.fill: parent

        AvatarMain {
            id: configAvatar
            iconColor: CmnCfg.palette.avatarColors[config.color]
            initials: config.name[0].toUpperCase()
            size: 28
            pfpPath: Utils.safeStringOrDefault(config.profilePicture, "")
            Layout.alignment: Qt.AlignCenter
            Layout.leftMargin: 12
            Layout.rightMargin: 12
            MouseArea {
                anchors.fill: parent
                id: avatarHoverHandler
                onPressed: {
                    overlay.visible = true
                }
                onReleased: {
                    overlay.visible = false
                }
                onClicked: { configPopup.show()
                }
            }
            ColorOverlay {
                id: overlay
                visible: false
                    anchors.fill: parent
                    source: parent
                    // hexquad black + transparent
                    color: "#40000000"
                    smooth: true
                }

        }

        Text {
            id: headerText
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
            spacing: 12
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
