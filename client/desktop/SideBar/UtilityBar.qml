import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups
import "../common" as Common
import "../common/js/utils.mjs" as Utils
import "../SideBar" as SideBar
import Qt.labs.platform 1.1

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
Component {
    ToolBar {
        id: utilityBar
        anchors.left: parent.left
        anchors.right: parent.right
        height: CmnCfg.toolbarHeight

        background: Rectangle {
            anchors.fill: parent
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
                size: parent.height - 2 * CmnCfg.margin
                isDefault: true
                inLayout: true
            }

            //probably need a standard divider that also handles layouts
            Rectangle {
                Layout.alignment: Qt.AlignHCenter
                height: parent.height
                width: 2
                color: CmnCfg.palette.mainColor
            }

            Text {
                text: "Conversations"
                font.pixelSize: CmnCfg.headerSize
                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                color: CmnCfg.palette.mainTextColor
            }

            Item {
                Layout.fillWidth: true
            }

            Common.ButtonForm {
                id: searchButton
                property bool searchRegex: false
                Layout.leftMargin: CmnCfg.smallMargin
                Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                //this is a vertical center offset
                Layout.topMargin: 1
                source: "qrc:/search-icon.svg"
                //todo : add back in regex logic once ui is known
                onClicked: {
                    convoPane.state = "conversationSearch"
                }
            }

            ///--- Add contact button
            Common.ButtonForm {
                id: newMessageButton
                Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                // Layout.leftMargin: CmnCfg.margin
                // Layout.rightMargin: CmnCfg.margin
                source: "qrc:/pencil-icon-black.svg"
                z: -1

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        convoPane.state = "newConversationState"
                    }
                }
            }

            //placeholder new contact button
            Common.ButtonForm {
                id: newContactButton

                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                //  Layout.leftMargin: CmnCfg.margin
                Layout.rightMargin: CmnCfg.margin
                source: "qrc:/options-icon.svg"

                MouseArea {
                    anchors.fill: parent

                    onClicked: {
                        utilityOptionsMenu.open()
                    }
                }
            }

            Menu {
                id: utilityOptionsMenu
                MenuItem {
                    text: "Add contact"
                    onTriggered: convoPane.state = "newContactState"
                }

                MenuItem {
                    text: "Config settings"
                    onTriggered: configPopup.show()
                }
            }
        }
    }
}
