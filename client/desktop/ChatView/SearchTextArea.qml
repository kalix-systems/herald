import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import QtQuick.Controls.Styles 1.0
import "Controls/" as CVUtils
import "../common" as Common
import "js/SearchHandler.mjs" as SearchUtils

ScrollView {
    Layout.maximumWidth: 300
    Layout.minimumWidth: 200
    Layout.alignment: Qt.AlignLeft
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
