import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "./ConversationView"
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

    Column {
      anchors.fill: parent
     Loader {
        id: sideBarBodyLoader
        sourceComponent: Component {
            ConversationViewMain {
                id: convosLvComponent
                model: conversationsModel}}
        width: parent.width
      }

    Loader {
        id: messageSearchLoader
        width: parent.width
        property var searchModel
        sourceComponent: Component {
            MessageSearchView {
                model: searchModel
            }
        }
    }
    }

    states: [
        State {
            name: "newContactState"
            PropertyChanges {
                target: sideBarBodyLoader
                sourceComponent: newContactComponent
            }
            PropertyChanges {
                target: headerLoader
                sourceComponent: headerBarComponent
                searchPlaceholder: "Search your conversations"
                headerText: "Add contact"
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
                headerText: "New group"
                contactsSearch: true
            }
            PropertyChanges {
                target: convoBuilderLoader
                active: true
                source: "GroupFlowComponents/ConvoBuilder.qml"
            }
        },

        State {
            name: "conversationSearch"
            PropertyChanges {
                target: headerLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: "Search your conversations"
            }

            PropertyChanges {
                target: searchModelLoader
                source: "MessageSearch.qml"
            }

            PropertyChanges {
                target: messageSearchLoader
                searchModel: msgSearchModel
            }

        },

        //TODO: following state should be reworked to match new design
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
        }
    ]
}
