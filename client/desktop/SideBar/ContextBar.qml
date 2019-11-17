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
import Qt.labs.platform 1.0

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
            pfpPath: Utils.safeStringOrDefault(config.profilePicture, "")

            size: 28
            Layout.alignment: Qt.AlignCenter
            Layout.leftMargin: 12
            Layout.rightMargin: 12
            avatarHeight: 28

            MouseArea {
                id: avatarHoverHandler
                anchors.fill: parent

                cursorShape: Qt.PointingHandCursor

                onPressed: overlay.visible = true
                onReleased: overlay.visible = false
                onClicked: configPopup.show()
            }

            ColorOverlay {
                id: overlay
                anchors.fill: parent
                source: parent

                visible: false
                color: "black"
                opacity: 0.2
                smooth: true
            }
        }

        Text {
            id: headerText
            text: "Conversations"
            font {
                pixelSize: CmnCfg.headerSize
                family: CmnCfg.chatFont.name
                bold: true
            }
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            color: CmnCfg.palette.mainColor
        }

        // filler item
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
                // this is a vertical center offset
                topPadding: 1
                source: "qrc:/search-icon.svg"
                // TODO : add back in regex logic once ui is known
                onClicked: sideBarState.state = "conversationSearch"
            }

            Common.ButtonForm {
                id: newMessageButton
                source: "qrc:/compose-icon-white.svg"
                fill: CmnCfg.palette.paneColor
                onClicked: convoMenu.open()
            }

            Menu {
                id: convoMenu
                MenuItem {
                    text: "New group conversation"
                    onTriggered: {
                        sideBarState.state = "newGroupState"
                    }
                }
            }

            Common.ButtonForm {
                id: optionsButton
                fill: CmnCfg.palette.paneColor
                source: "qrc:/options-icon.svg"
                onClicked: contextOptionsMenu.open()
            }
        }
    }
}
