import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "./ConversationView"
import "../SideBar/NewConvoComponents" as ConvUtils
import "../SideBar/GroupFlowComponents" as GroupFlow

Page {
    id: sideBarStateLoader
    padding: 0

    background: Rectangle {
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

   GroupFlow.NewGroupComponent {
       id: newGroupComponent
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
                target: headerLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Enter full name or username"
            }
        },

        State {
            name: "conversationSearch"
            PropertyChanges {
                target: headerLoader
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
                target: headerLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Enter contact name"
                contactsSearch: true
            }

            PropertyChanges {
                target: newGroupBar
                visible: true
            }

        },

        State {
            name: "newGroupState"
            PropertyChanges {
                target: sideBarBodyLoader
                sourceComponent: newGroupComponent
            }

            PropertyChanges {
                target: headerLoader
                sourceComponent: headerBarComponent
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
                target: headerLoader
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
