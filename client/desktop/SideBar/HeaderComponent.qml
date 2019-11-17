import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "../SideBar" as SideBar
import "qrc:/imports/Avatar"
import "popups" as Popups
import QtGraphicalEffects 1.0

//header component loaded during new group & new contact flow
Component {

    ToolBar {
        id: headerBarComponent
        height: CmnCfg.toolbarHeight
        background: Rectangle {
            color: CmnCfg.palette.secondaryColor
        }
        RowLayout {

            anchors.fill: parent

            AvatarMain {
                id: configAvatar
                iconColor: CmnCfg.palette.avatarColors[config.color]
                initials: config.name[0].toUpperCase()
                size: 28
                avatarHeight: 28
                pfpPath: Utils.safeStringOrDefault(config.profilePicture, "")
                Layout.alignment: Qt.AlignCenter
                Layout.leftMargin: 12
                Layout.rightMargin: 12
                MouseArea {
                    anchors.fill: parent
                    id: avatarHoverHandler
                    cursorShape: Qt.PointingHandCursor
                    onPressed: {
                        overlay.visible = true
                    }
                    onReleased: {
                        overlay.visible = false
                    }
                    onClicked: {
                        configPopup.show()
                    }
                }

                ColorOverlay {
                    id: overlay
                    visible: false
                    anchors.fill: parent
                    source: parent
                    color: "black"
                    opacity: 0.2
                    smooth: true
                }
            }

            Text {
                id: text
                text: headerLoader.headerText
                font.pixelSize: CmnCfg.headerSize
                font.family: CmnCfg.chatFont.name
                font.bold: true
                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                color: CmnCfg.palette.mainColor
            }
            Item {
                Layout.fillWidth: true
            }

            Common.ButtonForm {
                id: xButton
                fill: CmnCfg.palette.paneColor
                source: "qrc:/x-icon.svg"
                scale: 0.8
                onClicked: sideBarState.state = ""
            }
        }
    }
}
