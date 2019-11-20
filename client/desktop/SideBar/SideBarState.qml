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

    Flickable {
        anchors.fill: parent
        contentHeight: wrapperCol.height
        interactive: true
        ScrollBar.vertical: ScrollBar { policy: ScrollBar.AsNeeded
            width: CmnCfg.padding
            }
    //column to load content, components are inside instead of being declared separately because
    // otherwise loader cannot keep track of contentHeight of the listviews.
    Column {
     id: wrapperCol
     width: parent.width
     Text {
         text: "Conversations"
         anchors.left: parent.left
         anchors.leftMargin: CmnCfg.smallMargin
         topPadding: CmnCfg.smallMargin
         font.bold: true
         visible: sideBarState.state == "conversationSearch"
     }

     Loader {
        id: sideBarBodyLoader
        sourceComponent: Component {
            ConversationViewMain {
                id: convosLvComponent
                model: conversationsModel}}
        width: parent.width
        onWidthChanged: print("Width", width)
      }

     Text {
         text: "Messages"
         anchors.left: parent.left
         anchors.leftMargin: CmnCfg.smallMargin
         topPadding: CmnCfg.smallMargin
         font.bold: true
         visible: sideBarState.state == "conversationSearch"
     }

    Loader {
        id: messageSearchLoader
        width: parent.width
        //model loaded into search view only in search state
        property var searchModel
        sourceComponent: Component {
            MessageSearchView {
                model: searchModel
            }
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

            //loader that loads message search model
            PropertyChanges {
                //loader in sidebarMain
                target: searchModelLoader
                active: true
                source: "MessageSearch.qml"
            }

            //load model into view
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
