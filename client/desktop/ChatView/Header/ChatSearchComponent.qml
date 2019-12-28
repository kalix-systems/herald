import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import QtQuick.Controls.Styles 1.0
import "../../common" as Common
import "qrc:/imports" as Imports
import "../js/SearchHandler.mjs" as SearchUtils

Component {
    id: searchBarComponent

    Column {
        //wrapper column to position text field and underline
        anchors.right: parent.right
        RowLayout {

            id: searchToolBar
            anchors.horizontalCenter: parent.horizontalCenter

            spacing: CmnCfg.microMargin

            anchors {
                leftMargin: CmnCfg.defaultMargin
                rightMargin: CmnCfg.defaultMargin
            }

            //main search component
            SearchTextField {
                //TODO: proper width calculation
                Layout.maximumWidth: 300
                Layout.minimumWidth: 200
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            }

            Text {
                id: indexText
                property bool active: searchToolBar.state == "searchActiveState"
                property int searchPlace: active ? ownedConversation.searchIndex : 0
                property int numMatches: active ? ownedConversation.searchNumMatches : 0
                color: CmnCfg.palette.lightGrey
                font.family: CmnCfg.chatFont.name
                text: active ? searchPlace + "/" + numMatches : ""
                Layout.minimumWidth: 24
                Layout.leftMargin: -20
            }

            Imports.IconButton {
                id: back
                source: "qrc:/up-chevron-icon.svg"
                fill: CmnCfg.palette.lightGrey
                Layout.alignment: Qt.AlignVCenter
                enabled: searchToolBar.state === "searchActiveState"
                opacity: enabled ? 1 : 0.5
                onClicked: convWindow.positionViewAtIndex(
                               ownedConversation.prevSearchMatch(),
                               ListView.Center)
            }

            Imports.IconButton {
                id: forward
                source: "qrc:/down-chevron-icon.svg"
                fill: CmnCfg.palette.lightGrey
                Layout.alignment: Qt.AlignVCenter
                enabled: searchToolBar.state === "searchActiveState"
                opacity: enabled ? 1 : 0.5

                onClicked: convWindow.positionViewAtIndex(
                               ownedConversation.nextSearchMatch(),
                               ListView.Center)
            }

            Imports.IconButton {
                source: "qrc:/x-icon.svg"
                Layout.alignment: Qt.AlignVCenter
                fill: CmnCfg.palette.lightGrey
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
            width: searchToolBar.width - CmnCfg.microMargin
            anchors.left: parent.left
            color: "white"
        }

        Component.onDestruction: ownedConversation.searchActive = false
    }
}
