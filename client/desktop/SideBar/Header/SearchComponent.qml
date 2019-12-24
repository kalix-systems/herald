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

            Rectangle {
                Layout.fillWidth: true
                color: CmnCfg.palette.offBlack
                height: parent.height

                TextArea {
                    id: searchText
                    height: CmnCfg.toolbarHeight - CmnCfg.microMargin
                    width: parent.width
                    placeholderText: headerLoader.searchPlaceholder
                    color: "white"
                    verticalAlignment: TextEdit.AlignVCenter
                    anchors.top: parent.top
                    anchors.topMargin: CmnCfg.smallMargin / 4

                    Keys.onPressed: {
                        // this makes sure that returns and tabs are not evaluated
                        if (event.key === Qt.Key_Return
                                || event.key === Qt.Key_Tab) {
                            event.accepted = true
                        }
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

                Rectangle {
                    width: searchText.width - CmnCfg.largeMargin
                    color: "white"
                    height: 1
                    anchors.horizontalCenter: parent.horizontalCenter
                    anchors.bottomMargin: CmnCfg.smallMargin
                    anchors.bottom: parent.bottom
                }

            }

            Imports.ButtonForm {
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
