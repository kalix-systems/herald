import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "../SideBar/Header" as Header
import "../SideBar/Pane" as Pane
import "../SideBar/Pane/GroupFlowComponents" as GroupFlow

Page {
    id: sideBarStateLoader
    padding: 0

    background: Rectangle {
        color: CmnCfg.palette.lightGrey
    }

    Component {
        id: contactslvComponent
        Pane.ContactView {
            id: contactsListView
            anchors.fill: parent
            model: herald.users
        }
    }

    Pane.SideBarPane {
        id: sideBarPane
    }

    states: [
        State {
            name: "newContactState"
            PropertyChanges {
                target: sideBarPane.sideBarBodyLoader
                sourceComponent: newContactComponent
            }
            PropertyChanges {
                target: headerLoader
                sourceComponent: headerBarComponent
                searchPlaceholder: qsTr("Search your conversations")
                headerText: qsTr("Add contact")
            }
        },

        State {
            name: "newGroupState"
            PropertyChanges {
                target: sideBarPane.sideBarBodyLoader
                sourceComponent: newGroupComponent
            }

            PropertyChanges {
                target: headerLoader
                sourceComponent: headerBarComponent
                headerText: qsTr("New group")
                contactsSearch: true
            }
        },

        State {
            name: "globalSearch"
            PropertyChanges {
                target: headerLoader
                sourceComponent: searchBarComponent
                searchPlaceholder: qsTr("Search your conversations")
            }

            //load model into view
            PropertyChanges {
                target: sideBarPane.messageSearchLoader
                searchModel: herald.messageSearch
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
                searchPlaceholder: qsTr("Enter contact name")
                contactsSearch: true
            }
        }
    ]
}
