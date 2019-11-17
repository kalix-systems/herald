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

    Column {
       // width: parent.width
        anchors.right: parent.right
    RowLayout {
        Layout.maximumWidth: 400
        id: searchToolBar
        anchors.horizontalCenter: parent.horizontalCenter


        spacing: CmnCfg.smallMargin / 2

        anchors {
            leftMargin: CmnCfg.margin
            rightMargin: CmnCfg.margin
        }

        ScrollView {
            Layout.maximumWidth: 300
            Layout.minimumWidth: 200
            clip: true
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
                var isOnscreen = SearchUtils.isOnscreen(ownedConversation, convWindow.chatListView,
                                                        chatPane, convWindow, false)

                if (!isOnscreen) {
                convWindow.contentY =
                        convWindow.chatListView.itemAt(ownedConversation.prevSearchMatch()).y - convWindow.height / 2
                    convWindow.returnToBounds()
            }
            }

            else {print("no matches")
                searchToolBar.state = ""
                }
                }
    }

        }


    Common.ButtonForm {
        id: back
        source: "qrc:/up-chevron-icon-white.svg"
       Layout.alignment: Qt.AlignVCenter
       fill: CmnCfg.palette.paneColor
       enabled: searchToolBar.state === "searchActiveState"
       opacity: enabled ? 1 : 0.5
       onClicked: {
           SearchUtils.jumpHandler(ownedConversation, convWindow.chatListView, chatPane, convWindow, false)
           convWindow.returnToBounds()
       }
    }

    Common.ButtonForm {
        id: forward
        source: "qrc:/down-chevron-icon-white.svg"
       Layout.alignment: Qt.AlignVCenter
       fill: CmnCfg.palette.paneColor
       enabled: searchToolBar.state === "searchActiveState"
       opacity: enabled ? 1 : 0.5

       onClicked: {
           SearchUtils.jumpHandler(ownedConversation, convWindow.chatListView, chatPane, convWindow, true)
           convWindow.returnToBounds()
       }
    }

    Common.ButtonForm {
        source: "qrc:/x-icon.svg"
       Layout.alignment: Qt.AlignVCenter
       fill: CmnCfg.palette.paneColor
        onClicked: {
            ownedConversation.clearSearch()
            ownedConversation.searchActive = false
            messageBar.state = ""
        }
        scale: 0.8
    }

    states: State {
        name: "searchActiveState"
    }


    }

    Rectangle {
        height: 1
        width: searchToolBar.width - CmnCfg.smallMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "white"
    }
  }

}


