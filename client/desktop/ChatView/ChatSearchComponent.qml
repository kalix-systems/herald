import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import QtQuick.Controls.Styles 1.0
import "Controls/" as CVUtils
import "../common" as Common
import "js/SearchHandler.mjs" as SearchUtils

Component {
    id: searchBarComponent

    Column {
        //wrapper column to position textarea and underline
        anchors.right: parent.right
        RowLayout {
            id: searchToolBar
            anchors.horizontalCenter: parent.horizontalCenter

            spacing: CmnCfg.smallMargin / 2

            anchors {
                leftMargin: CmnCfg.margin
                rightMargin: CmnCfg.margin
            }

            //main search component
            SearchTextArea {
            }

            Text {
                id: indexText
                property bool active: searchToolBar.state == "searchActiveState"
                property int searchPlace: active ? ownedConversation.searchIndex : 0
                property int numMatches: active ? ownedConversation.searchNumMatches : 0

                color: CmnCfg.palette.paneColor
                font.family: CmnCfg.chatFont.name
                text: active ? searchPlace + "/" + numMatches : ""
                Layout.minimumWidth: 24
            }

            Common.ButtonForm {
                id: back
                source: "qrc:/up-chevron-icon-white.svg"
                Layout.alignment: Qt.AlignVCenter
                fill: CmnCfg.palette.paneColor
                enabled: searchToolBar.state === "searchActiveState"
                opacity: enabled ? 1 : 0.5
                onClicked: {
                    convWindow.state = "jumpState"
                    SearchUtils.jumpHandler(ownedConversation,
                                            convWindow.chatListView, chatPane,
                                            convWindow, false)
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
                    convWindow.state = "jumpState"
                    SearchUtils.jumpHandler(ownedConversation,
                                            convWindow.chatListView, chatPane,
                                            convWindow, true)
                    convWindow.returnToBounds()
                }
            }

            Common.ButtonForm {
                source: "qrc:/x-icon.svg"
                Layout.alignment: Qt.AlignVCenter
                fill: CmnCfg.palette.paneColor
                onClicked: {
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
