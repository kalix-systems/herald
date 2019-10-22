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
        height: QmlCfg.toolbarHeight

        background: Rectangle {
            anchors.fill: parent
            color: QmlCfg.palette.secondaryColor
        }

        RowLayout {
            anchors.fill: parent

            Common.Avatar {
                id: configAvatar
                Layout.topMargin: QmlCfg.smallMargin
                Layout.rightMargin: QmlCfg.smallMargin
                Layout.leftMargin: QmlCfg.smallMargin
                Layout.alignment: Qt.AlignVCenter | Qt.AlignTop | Qt.AlignLeft | Qt.AlignHCenter
                avatarLabel: config.name
                labeled: false
                colorHash: config.color
                pfpUrl: Utils.safeStringOrDefault(config.profilePicture, "")
                labelGap: 0
                // JH: Bad margin semantics
                size: parent.height - 2 * QmlCfg.margin
                isDefault: true
                inLayout: true
            }

            //probably need a standard divider that also handles layouts
            Rectangle {
                Layout.alignment: Qt.AlignHCenter
                height: parent.height
                width: 2
                color: QmlCfg.palette.mainColor
            }

            Text {
                text: "Conversations"
                font.pixelSize: QmlCfg.headerSize
                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                color: QmlCfg.palette.mainTextColor
            }

            Item {
                Layout.fillWidth: true
            }

            Common.ButtonForm {
                id: searchButton
                property bool searchRegex: false
                Layout.leftMargin: QmlCfg.smallMargin
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
                // Layout.leftMargin: QmlCfg.margin
                // Layout.rightMargin: QmlCfg.margin
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
                //  Layout.leftMargin: QmlCfg.margin
                Layout.rightMargin: QmlCfg.margin
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
