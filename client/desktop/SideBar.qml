import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "SideBar" as SBUtils
import "SideBar/NewConvoComponents" as ConvUtils
import "common" as Common

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
Pane {
    id: contactPane
    // GS : we do this to get the current Item, BAD.
    //    property alias contactsListView: contactsListView
    //    property alias conversationsListView: conversationsListView
    property real windowFraction: width / root.width
    readonly property real maxWindowFraction: 0.66
    // maximum width, where root is ApplicationWindow
    SplitView.maximumWidth: root.width * maxWindowFraction
    SplitView.minimumWidth: 250
    SplitView.preferredWidth: root.width * windowFraction
    property alias groupMemberSelect: convoBuilderLoader.item

    padding: 0 // All Interior Elements span the entire pane
    height: parent.height

    background: Rectangle {
        border.color: QmlCfg.palette.secondaryColor
    }

    ///--- Username and Settings gear button
    SBUtils.ConfigBar {
        id: toolBar
    }

    Common.Divider {
        id: configBarBorder
        anchors.bottom: utilityBar.top
        color: QmlCfg.palette.secondaryColor
        height: 1
    }

    ///--- SearchBar for contacts, add contact button
    Column {
        id: utilityBar
        anchors.top: toolBar.bottom
        width: parent.width
        Loader {
            property string searchPlaceholder: ""
            property bool contactsSearch: false
            id: searchLoader
            sourceComponent: utilityBarComponent
            width: parent.width
        }
    }

    SBUtils.UtilityBar {
        id: utilityBarComponent
    }

    //search component loaded to search convos and contacts
    SBUtils.SearchComponent {
        id: searchBarComponent
    }

    //component loaded when selecting a new group
    SBUtils.GroupSelectComponent {
        id: groupSelectComponent
    }

    //component loaded when finalizing new group
    SBUtils.FinalizeGroupComponent {
        id: finalizeGroupComponent
    }

    ///--- Border between SearchBar and the Pane Contents (contacts)
    Common.Divider {
        id: searchBarBorder
        anchors.top: utilityBar.bottom
        color: "black"
    }

    SBUtils.NewGroupBar {
        id: newGroupBar
        anchors.top: searchBarBorder.bottom
        visible: (convoPane.state == "newConversationState")
    }

    Loader {
        id: convoBuilderLoader
    }

    ///--- Contacts View Actual
    Pane {
        id: convoPane
        padding: 0
        anchors {
            right: parent.right
            left: parent.left
            top: newGroupBar.bottom
            bottom: parent.bottom
        }

        Component {
            id: contactslvComponent
            SBUtils.ContactView {
                id: contactsListView
                anchors.fill: parent
                model: contactsModel
            }
        }

        Component {
            id: convoslvComponent
            SBUtils.ConversationView {
                id: conversationsListView
                anchors.fill: parent
                model: conversationsModel
            }
        }

        Component {
            id: convoFinalGroup
            SBUtils.FinalGroupList {
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
                    target: convoPane
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
                    source: "SideBar/NewConvoComponents/ConvoBuilder.qml"
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
                    source: "SideBar/NewConvoComponents/ConvoBuilder.qml"
                }

                PropertyChanges {
                    target: sideBarBodyLoader
                    sourceComponent: convoFinalGroup
                }
            }
        ]
    }
}
