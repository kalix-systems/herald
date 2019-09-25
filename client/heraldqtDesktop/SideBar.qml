import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "SideBar" as SBUtils
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
    property alias contactsListView: contactsListView
    property alias conversationsListView: conversationsListView
    property real windowFraction: width / root.width
    readonly property real maxWindowFraction: 0.66
    // maximum width, where root is ApplicationWindow
    SplitView.maximumWidth: root.width * maxWindowFraction
    SplitView.minimumWidth: 250
    SplitView.preferredWidth: root.width * windowFraction

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
        color: "black"
        height: 1
    }

    ///--- SearchBar for contacts, add contact button
    SBUtils.UtilityBar {
        id: utilityBar
        anchors.top: toolBar.bottom

        Loader {
            property string searchPlaceholder: ""
            property bool contactsSearch: false
            anchors.fill: parent
            id: searchLoader
        }
    }

    SBUtils.SearchComponent {
        id: searchBarComponent
    }

    ///--- Border between SearchBar and the Pane Contents (contacts)
    Common.Divider {
        id: searchBarBorder
        anchors.top: utilityBar.bottom
        color: "black"
        height: 1.5
    }



    SBUtils.ContactsToggle {
        id: contactsToggleBar
        anchors.top: searchBarBorder.bottom
    }


    ///--- Contacts View Actual
    Pane {
        id: convoPane
        padding: 0
        anchors {
            right: parent.right
            left: parent.left
            top: contactsToggleBar.bottom
            bottom: parent.bottom
        }

        SBUtils.ContactView {
            id: contactsListView
            visible: false
            anchors.fill: parent
            model: contactsModel
        }

        SBUtils.ConversationView {
            id: conversationsListView
            anchors.fill: parent
            model: conversationsModel
        }

        states: [ State {name: "newContactState"
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

            State {name: "conversationSearch"
                PropertyChanges {
                    target: conversationsListView
                    visible: true
                }
                PropertyChanges {
                    target: searchLoader
                    sourceComponent: searchBarComponent
                    searchPlaceholder: "Search your conversations"
                }

            },

            State {name: "newConversationState"
                PropertyChanges {
                    target: contactsListView
                    visible: true

                }
                PropertyChanges {
                    target: conversationsListView
                    visible: false
                }

                PropertyChanges {
                    target: searchLoader
                    sourceComponent: searchBarComponent
                    searchPlaceholder: "Enter contact name"
                    contactsSearch: true
                }

            }


        ]
    }
}
