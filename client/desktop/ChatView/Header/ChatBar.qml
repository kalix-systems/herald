import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import Qt.labs.platform 1.0
import "qrc:/imports" as Imports
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils
import "../../common" as Common

ToolBar {
    id: chatToolBar
    property var conversationItem
    property Messages ownedConversation: parent.ownedConversation

    height: CmnCfg.toolbarHeight
    z: CmnCfg.middleZ

    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }
    Rectangle {
        anchors.left: parent.left
        height: parent.height + 1
        width: 1
        color: CmnCfg.palette.lightGrey
    }

    RowLayout {
        id: buttonRow

        spacing: 12

        anchors {
            fill: parent
            leftMargin: CmnCfg.defaultMargin
            rightMargin: CmnCfg.smallMargin
        }

        Avatar {
            id: avatar
            diameter: 32
            color: CmnCfg.avatarColors[conversationItem.color]
            initials: conversationItem.title[0].toUpperCase()
            Layout.alignment: Qt.AlignLeft
            pfpPath: Utils.safeStringOrDefault(conversationItem.picture, "")
            isGroup: !conversationItem.pairwise
            anchors {
                margins: CmnCfg.defaultMargin
            }
        }

        Label {
            id: uid
            font: CmnCfg.headerBarFont
            Layout.alignment: Qt.AlignLeft
            Layout.fillWidth: true
            elide: Label.ElideRight
            text: conversationItem.title
            color: CmnCfg.palette.white
            // top padding aligns headerText baseline with baseline of
            // initial in user avatar to right
            topPadding: 1
        }

        Loader {
            id: searchLoader
            Layout.alignment: Qt.AlignLeft
            height: parent.height
        }

        Row {
            id: optionsRow
            spacing: CmnCfg.defaultMargin
            Layout.alignment: Qt.AlignRight
            height: parent.height

            Imports.ButtonForm {
                id: searchButton
                source: "qrc:/search-icon.svg"
                fill: CmnCfg.palette.lightGrey
                topPadding: 1
                onClicked: chatToolBar.state = "searchState"
            }

            Imports.ButtonForm {
                id: timerButton
                source: timerMenu.chosenTimer
                fill: "transparent"
                topPadding: 1
                onClicked: timerMenu.open()
            }

            Imports.TimerOptions {
                id: timerMenu
                conversationItem: chatToolBar.conversationItem
            }

            Imports.ButtonForm {
                id: convOptionsButton
                source: "qrc:/options-icon.svg"
                fill: CmnCfg.palette.lightGrey
                onClicked: convOptionsMenu.open()
                Menu {
                    id: convOptionsMenu

                    MenuItem {
                        text: qsTr("Archive")
                    }

                    MenuItem {
                        text: qsTr("Clear History")
                        onTriggered: ownedConversation.clearConversationHistory(
                                         )
                    }
                }
            }
        }
    }

    states: State {
        name: "searchState"

        PropertyChanges {
            target: searchButton
            visible: false
        }

        PropertyChanges {
            target: searchLoader
            sourceComponent: chatSearchComponent
        }
    }
}
