import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "../popups/js/NewContactDialogue.mjs" as JS
import "../../SideBar" as SBUtils
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Avatar"
import "qrc:/imports" as Imports
import "qrc:/imports/js/utils.mjs" as Utils

Component {
    id: searchBarComponent

    ToolBar {
        height: CmnCfg.toolbarHeight
        background: Rectangle {
            color: CmnCfg.palette.secondaryColor
        }
        RowLayout {

            anchors.fill: parent

            Common.ConfigAvatar {
            }

            Rectangle {
                Layout.fillWidth: true
                color: CmnCfg.palette.secondaryColor
                height: parent.height
                Layout.leftMargin: -12

                TextArea {
                    id: searchText
                    height: CmnCfg.toolbarHeight - CmnCfg.smallMargin / 2
                    width: parent.width
                    placeholderText: headerLoader.searchPlaceholder
                    color: "white"
                    verticalAlignment: TextEdit.AlignVCenter
                    anchors.top: parent.top
                    anchors.topMargin: CmnCfg.smallMargin / 4

                    background: Rectangle {
                        color: CmnCfg.palette.secondaryColor
                        anchors.fill: parent
                    }

                    Keys.onPressed: {
                        // this makes sure that returns and tabs are not evaluated
                        if (event.key === Qt.Key_Return
                                || event.key === Qt.Key_Tab) {
                            event.accepted = true
                        }
                    }

                    Imports.ButtonForm {
                        source: "qrc:/x-icon.svg"
                        scale: 0.8
                        fill: CmnCfg.palette.paneColor
                        anchors.right: parent.right
                        anchors.rightMargin: CmnCfg.smallMargin / 2
                        anchors.verticalCenter: parent.verticalCenter
                        onClicked: {
                            herald.messageSearch.clearSearch()
                            herald.conversations.filter = ""
                            sideBarState.state = ""
                        }
                    }

                    onTextChanged: {
                        if (contactsSearch) {
                            Qt.callLater(function (text) {
                                herald.users.filter = text
                            }, searchText.text)
                        } else {
                            Qt.callLater(function (text) {
                                herald.conversations.filter = text
                                herald.messageSearch.searchPattern = text
                            }, searchText.text)
                        }
                    }

                    Component.onDestruction: {
                        herald.users.clearFilter()
                        herald.conversations.clearFilter()
                        herald.messageSearch.clearSearch()
                    }
                }

                Rectangle {
                    width: searchText.width - CmnCfg.mediumMargin
                    color: "white"
                    height: 1
                    anchors.horizontalCenter: parent.horizontalCenter
                    anchors.bottomMargin: CmnCfg.smallMargin
                    anchors.bottom: parent.bottom
                }
            }
        }
    }
}
