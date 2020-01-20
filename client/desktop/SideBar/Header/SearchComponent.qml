import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "../popups/js/NewContactDialogue.mjs" as JS
import "../../SideBar" as SBUtils
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Entity"
import "qrc:/imports" as Imports
import "qrc:/imports/js/utils.mjs" as Utils

Component {
    id: searchBarComponent

    ToolBar {
        height: CmnCfg.toolbarHeight + 1
        background: Rectangle {
            color: CmnCfg.palette.offBlack
        }

        RowLayout {
            anchors.fill: parent
            spacing: 0

            HeaderAvatar {
                Layout.leftMargin: CmnCfg.smallMargin
            }

            Imports.BorderedTextField {
                id: searchText
                placeholderText: headerLoader.searchPlaceholder
                selectByMouse: true

                font.pixelSize: CmnCfg.chatTextSize
                Layout.alignment: Qt.AlignBottom | Qt.AlignHCenter
                Layout.fillWidth: true
                Layout.bottomMargin: CmnCfg.smallMargin
                Layout.leftMargin: CmnCfg.smallMargin
                Layout.rightMargin: CmnCfg.smallMargin

                Keys.onPressed: {
                    // this makes sure that returns and tabs are not evaluated
                    if (event.key === Qt.Key_Return
                            || event.key === Qt.Key_Tab) {
                        event.accepted = true
                    }
                }

                Common.TextContextMenu {
                    parentText: parent
                }

                onTextChanged: {
                    if (contactsSearch) {
                        Qt.callLater(function (text) {
                            Herald.users.filter = text
                        }, searchText.text)
                    } else {
                        Qt.callLater(function (text) {
                            Herald.conversations.filter = text
                            Herald.messageSearch.searchPattern = text
                        }, searchText.text)
                    }
                }

                Component.onDestruction: {
                    Herald.users.clearFilter()
                    Herald.conversations.clearFilter()
                    Herald.messageSearch.clearSearch()
                }
            }

            Imports.IconButton {
                source: "qrc:/x-icon.svg"
                fill: CmnCfg.palette.lightGrey
                Layout.alignment: Qt.AlignRight
                Layout.rightMargin: CmnCfg.smallMargin
                onClicked: {
                    Herald.messageSearch.clearSearch()
                    Herald.conversations.filter = ""
                    sideBarState.state = ""
                }
            }
        }
    }
}
