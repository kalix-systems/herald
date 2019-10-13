import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups
import "../common" as Common
import "../common/js/utils.mjs" as Utils
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

        RowLayout {
            anchors.fill: parent

        Text {
            text: "Conversations"
            Layout.leftMargin: QmlCfg.margin
            Layout.rightMargin: QmlCfg.margin
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            color: QmlCfg.palette.mainTextColor
        }

        Common.ButtonForm {
            id: searchButton
            property bool searchRegex: false
            Layout.leftMargin: QmlCfg.margin
            Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
            //this is a vertical center offset
            Layout.topMargin: 1
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
            Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
            Layout.leftMargin: QmlCfg.margin
            source: "qrc:/pencil-icon-black.svg"
            z: -1

            MouseArea {
                anchors.fill: parent

                onClicked: {
                    convoPane.state = "newConversationState"
                }
            }
        }
    }
    }
}
