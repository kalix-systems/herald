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
    //TODO: proper width calculation
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
            color: CmnCfg.palette.offBlack
        }

        verticalAlignment: TextEdit.AlignVCenter
        Layout.alignment: Qt.AlignLeft

        Keys.onReturnPressed: {
            //don't allow enter key to affect textarea
            event.accepted = true
            ownedConversation.searchActive = true
            var x = convWindow.chatScrollBar.position
            var y = convWindow.chatScrollBar.size
            //key navigation handling
            if (ownedConversation.searchNumMatches > 0) {
                ownedConversation.setSearchHint(x, y)
                searchToolBar.state = "searchActiveState"
                convWindow.positionViewAtIndex(ownedConversation.nextSearchMatch(), ListView.Center)
            }
        }

        onTextChanged: {
            ownedConversation.searchActive = true
            ownedConversation.searchPattern = searchText.text
            var x = convWindow.chatScrollBar.position
            var y = convWindow.chatScrollBar.size
            ownedConversation.setSearchHint(x, y)
            if (ownedConversation.searchNumMatches > 0) {
                searchToolBar.state = "searchActiveState"
                convWindow.positionViewAtIndex(ownedConversation.prevSearchMatch(), ListView.Center)
            } else {
                //clear state to disable buttons
                searchToolBar.state = ""
            }
        }
    }
}
