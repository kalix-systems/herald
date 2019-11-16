import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import QtQuick.Controls.Styles 1.0
import "Controls/" as CVUtils
import "../common" as Common
import "../SideBar" as SBUtils
import "qrc:/imports/Avatar" as Avatar
import "js/SearchHandler.mjs" as SearchUtils

Component {
    id: searchBarComponent

    ToolBar {
        id: searchToolBar
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

        placeholderText: "Search conversation..."
        font.pixelSize: 14
        color: "white"
        background: Rectangle {
            anchors.fill: parent
            color: CmnCfg.palette.secondaryColor
        }

        verticalAlignment: TextEdit.AlignVCenter
        Layout.alignment: Qt.AlignLeft

        onTextChanged: {
            ownedConversation.searchActive = true
            ownedConversation.searchPattern = searchText.text

            if (ownedConversation.searchNumMatches > 0) {
                searchToolBar.state = "searchActiveState"
                convWindow.state = "searchState"
                var isOnscreen = SearchUtils.isOnscreen(ownedConversation, convWindow.chatListView, chatPane, false)

                if (!isOnscreen) {
                convWindow.contentY =
                        convWindow.chatListView.itemAt(ownedConversation.prevSearchMatch()).y
                    convWindow.returnToBounds()
            }
            }


            else {print("no matches")
                searchToolBar.state = ""
                }
                }
    }

    Common.ButtonForm {
        source: "qrc:/x-icon.svg"
       Layout.alignment: Qt.AlignVCenter
       fill: CmnCfg.palette.paneColor
        onClicked: {
            ownedConversation.clearSearch()
            ownedConversation.searchActive = false
            messageBar.sourceComponent = chatBarComponent
        }
        scale: 0.8
    }

    Common.ButtonForm {
        id: back
        source: "qrc:/back-arrow-icon.svg"
       Layout.alignment: Qt.AlignVCenter
       fill: CmnCfg.palette.paneColor
       enabled: searchToolBar.state === "searchActiveState"
       onClicked: {
           SearchUtils.jumpHandler(ownedConversation, convWindow.chatListView, chatPane, convWindow, false)
           convWindow.returnToBounds()
       }
    }

    Common.ButtonForm {
        id: forward
        source: "qrc:/forward-arrow-icon.svg"
       Layout.alignment: Qt.AlignVCenter
       fill: CmnCfg.palette.paneColor
       enabled: searchToolBar.state === "searchActiveState"

       onClicked: {
           SearchUtils.jumpHandler(ownedConversation, convWindow.chatListView, chatPane, convWindow, true)
           convWindow.returnToBounds()
       }
    }
    }

    Rectangle {
        height: 1
        anchors.right: parent.right
        anchors.rightMargin: 84
        anchors.bottom: parent.bottom
        anchors.bottomMargin: CmnCfg.smallMargin / 2
        width: searchText.width + 20
        color: CmnCfg.palette.paneColor
    }
    states: State {
        name: "searchActiveState"
    }
    }
}
