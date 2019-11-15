import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "../SideBar" as SBUtils
import "qrc:/imports/Avatar" as Avatar

Component {
    id: searchBarComponent

    ToolBar {
        height: CmnCfg.toolbarHeight
        z: CmnCfg.middleZ

        background: Rectangle {
            color: CmnCfg.palette.secondaryColor
        }

    RowLayout {
        id: buttonRow

        spacing: 12

        anchors {
            fill: parent
            leftMargin: CmnCfg.margin
            rightMargin: CmnCfg.margin
        }

        Avatar.AvatarMain {

            size: 32
            iconColor: CmnCfg.avatarColors[conversationItem.color]
            textColor: CmnCfg.palette.iconFill
            initials: conversationItem.title[0].toUpperCase()
            Layout.alignment: Qt.AlignLeft
            anchors {
                margins: 16
            }
        }

        Label {
            id: uid
            font {
                bold: true
                family: CmnCfg.chatFont.name
                pixelSize: 18
            }
            Layout.alignment: Qt.AlignLeft
            Layout.fillWidth: true
            elide: "ElideRight"
            text: conversationItem.title
            color: "white"
        }

    TextArea {
        id: searchText
        height: CmnCfg.toolbarHeight
        Layout.fillWidth: true

        placeholderText: "Search conversation..."
        color: CmnCfg.palette.mainTextColor
        verticalAlignment: TextEdit.AlignVCenter
        Layout.alignment: Qt.AlignLeft

        onTextChanged: {
            ownedConversation.searchActive = true
            ownedConversation.searchPattern = searchText.text

            print(ownedConversation.searchNumMatches, ownedConversation.prevSearchMatch())
        }

        Common.ButtonForm {
            source: "qrc:/x-icon.svg"
            height: 20
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
        }
        background: Rectangle {
            anchors.fill: parent
            color: "white"
        }
    }
    }
    }
}
