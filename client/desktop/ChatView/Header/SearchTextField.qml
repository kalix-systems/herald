import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import QtQuick.Controls.Styles 1.0
import "../../common" as Common
import "qrc:/imports" as Imports
import "../js/SearchHandler.mjs" as SearchUtils

ScrollView {
    clip: true
    ScrollBar.horizontal: ScrollBar {
        policy: ScrollBar.AlwaysOff
    }

    TextArea {
        id: searchText
        height: CmnCfg.toolbarHeight

        placeholderText: "Search conversation"
        font.pixelSize: 14
        color: CmnCfg.palette.white
        leftPadding: 0
        bottomPadding: 0
        selectByMouse: true

        verticalAlignment: TextEdit.AlignTop

        Keys.onReturnPressed: {
            const backwards = (event.modifiers & Qt.ShiftModifier)
            //don't allow enter key to affect textfield
            event.accepted = true

            if (searchText.text.length === 1) {
                ownedConversation.searchActive = false
                searchToolBar.state = ""
                return
            }
            ownedConversation.searchActive = true

            const x = convWindow.chatScrollBar.position
            const y = convWindow.chatScrollBar.size

            //key navigation handling
            if (ownedConversation.searchNumMatches > 0) {
                ownedConversation.setSearchHint(x, y)
                searchToolBar.state = "searchActiveState"

                if (backwards) {
                    convWindow.positionViewAtIndex(
                                ownedConversation.prevSearchMatch(),
                                ListView.Center)
                } else {
                    convWindow.positionViewAtIndex(
                                ownedConversation.nextSearchMatch(),
                                ListView.Center)
                }
            }
        }

        onTextChanged: {
            if (searchText.text.length === 1) {
                ownedConversation.searchActive = false
                searchToolBar.state = ""
                return
            }

            ownedConversation.searchActive = true
            ownedConversation.searchPattern = searchText.text

            const x = convWindow.chatScrollBar.position
            const y = convWindow.chatScrollBar.size

            ownedConversation.setSearchHint(x, y)

            if (ownedConversation.searchNumMatches > 0) {
                searchToolBar.state = "searchActiveState"
                convWindow.positionViewAtIndex(
                            ownedConversation.prevSearchMatch(),
                            ListView.Center)
            } else {
                //clear state to disable buttons
                searchToolBar.state = ""
            }
        }
    }
}
