import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "popups/js/NewContactDialogue.mjs" as JS
import "../SideBar" as SBUtils

Component {
    id: searchBarComponent

    TextArea {
        id: searchText
        height: CmnCfg.toolbarHeight

        placeholderText: headerLoader.searchPlaceholder
        color: CmnCfg.palette.mainTextColor
        verticalAlignment: TextEdit.AlignVCenter

        background: Rectangle {
            color: CmnCfg.palette.mainColor
            anchors.fill: parent
        }

        Keys.onPressed: {
            // this makes sure that returns and tabs are not evaluated
            if (event.key === Qt.Key_Return || event.key === Qt.Key_Tab) {
                event.accepted = true
            }
        }

        Common.ButtonForm {
            source: "qrc:/x-icon.svg"
            height: 20
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            onClicked: {
                sideBarState.state = ""
            }
        }

        onTextChanged: {
            if (contactsSearch) {
                Qt.callLater(function (text) {
                    contactsModel.filter = text
                }, searchText.text)
            } else {
                Qt.callLater(function (text) {
                    conversationsModel.filter = text
                }, searchText.text)
            }
        }

        Component.onDestruction: {
            contactsModel.clearFilter()
            conversationsModel.clearFilter()
        }

        Keys.onReturnPressed: {
            if (sideBarState.state == "newContactState") {
                JS.insertContact(searchText, contactsModel)
                sideBarState.state = ""
            }
        }
    }
}
