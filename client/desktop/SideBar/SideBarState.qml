import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "./ConversationView"
import "../SideBar/NewConvoComponents" as ConvUtils

Pane {
    id: sideBarStateLoader
    anchors.fill: parent
    padding: 0

    background: Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.paneColor
    }

    Component {
        id: contactslvComponent
        ContactView {
            id: contactsListView
            anchors.fill: parent
            model: contactsModel
        }
    }

    Component {
        id: convoslvComponent
        ConversationViewMain {
            id: conversationsListView
            anchors.fill: parent
            model: conversationsModel
        }
    }

    Component {
        id: convoFinalGroup
        ConvUtils.FinalGroupList {
            id: groupListView
            anchors.fill: parent
            model: groupMemberSelect
        }
    }

    Loader {
        id: sideBarBodyLoader
        anchors.fill: parent
        sourceComponent: convoslvComponent
    }

    states: [
        State {
            name: "newContactState"
            PropertyChanges {
                target: sideBarStateLoader
                visible: false
            }
            PropertyChanges {
                target: searchLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Enter full name or username"
            }
        },

        State {
            name: "conversationSearch"
            PropertyChanges {
                target: searchLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Search your conversations"
            }
        },

        State {
            name: "newConversationState"
            PropertyChanges {
                target: sideBarBodyLoader
                sourceComponent: contactslvComponent
            }

            PropertyChanges {
                target: searchLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Enter contact name"
                contactsSearch: true
            }
        },

        State {
            name: "newGroupState"
            PropertyChanges {
                target: sideBarBodyLoader
                sourceComponent: contactslvComponent
            }

            PropertyChanges {
                target: searchLoader
                sourceComponent: groupSelectComponent
                contactsSearch: true
            }
            PropertyChanges {
                target: convoBuilderLoader
                source: "NewConvoComponents/ConvoBuilder.qml"
            }
        },

        State {
            name: "finalizeGroupState"

            PropertyChanges {
                target: searchLoader
                sourceComponent: finalizeGroupComponent
            }

            PropertyChanges {
                target: convoBuilderLoader
                source: "../SideBar/NewConvoComponents/ConvoBuilder.qml"
            }

            PropertyChanges {
                target: sideBarBodyLoader
                sourceComponent: convoFinalGroup
            }
        }
    ]
}
