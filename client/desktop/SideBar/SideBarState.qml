import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "../SideBar/Header" as Header
import "../SideBar/Pane" as Pane
import "../SideBar/Pane/GroupFlowComponents" as GroupFlow

Item {

    // TODO PAUL: add transitions
    states: [
        State {
            name: "newContactState"
            PropertyChanges {
                target: sideBarPane.sideBarBodyLoader
                sourceComponent: newContactComponent
            }
            PropertyChanges {
                target: headerLoader
                sourceComponent: altContextHeader
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
                sourceComponent: altContextHeader
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
                searchModel: Herald.messageSearch
            }
        },

        State {
            name: "archivedState"
            PropertyChanges {
                target: headerLoader
                sourceComponent: altContextHeader
                headerText: qsTr("Archive")
            }
            PropertyChanges {
                target: sideBarPane.sideBarBodyLoader
                archiveState: true
            }
        }
    ]
}
