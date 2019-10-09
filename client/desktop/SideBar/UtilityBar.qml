import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups
import "../common" as Common
import "../common/utils.mjs" as Utils
import "../SideBar" as SideBar

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

        Text {
            text: "Conversations"
            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.margins: QmlCfg.margin
            color: QmlCfg.palette.mainTextColor
        }

        Common.ButtonForm {
            id: searchButton
            property bool searchRegex: false
            anchors {
                right: addContactButton.left
                verticalCenter: parent.verticalCenter
                rightMargin: QmlCfg.margin
                verticalCenterOffset: 1
            }
            source: "qrc:/search-icon.svg"
            scale: 1.0
            //todo : add back in regex logic once ui is known
            onClicked: {
                convoPane.state = "conversationSearch"
            }
        }

        ///--- Add contact button
        Common.ButtonForm {
            id: addContactButton
            anchors {
                rightMargin: QmlCfg.margin
                verticalCenterOffset: 0
                right: parent.right
                verticalCenter: parent.verticalCenter
            }
            source: "qrc:/pencil-icon-black.svg"
            z: -1

            MouseArea {
                anchors.fill: parent

                onClicked: {
                    convoPane.state = "newConversationState"
                }
            }
        }

        //NOTE: see previous notes about using native dialogs
        // we're not using this anymore
        /**
        Popups.NewContactDialogue {
            id: newContactDialogue
        }
        **/
    }
}
