import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "popups/js/NewContactDialogue.mjs" as JS
import "../SideBar" as SBUtils
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Avatar"
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

                    Common.ButtonForm {
                        source: "qrc:/x-icon.svg"
                        scale: 0.8
                        fill: CmnCfg.palette.paneColor
                        anchors.right: parent.right
                        anchors.rightMargin: CmnCfg.smallMargin / 2
                        anchors.verticalCenter: parent.verticalCenter
                        onClicked: {
                            msgSearchModel.clearSearch()
                            conversationsModel.filter = ""
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
                                msgSearchModel.searchPattern = text
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
