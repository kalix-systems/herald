import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "NewConvoComponents" as ConvUtils
import "qrc:/common" as Common
import "../SideBar/GroupFlowComponents" as GroupFlow

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
Page {
    id: sideBar
    property real windowFraction: width / root.width
    readonly property real maxWindowFraction: 0.66
    // maximum width, where root is ApplicationWindow
    SplitView.maximumWidth: root.width * maxWindowFraction
    SplitView.minimumWidth: 250
    SplitView.preferredWidth: root.width * windowFraction
    property alias groupMemberSelect: convoBuilderLoader.item
    padding: 0 // All Interior Elements span the entire pane

    background: Rectangle {
        color: CmnCfg.palette.paneColor
    }

    ///--- SearchBar for contacts, add contact button
    header: Loader {
        id: headerLoader
        property string searchPlaceholder: ""
        property bool contactsSearch: false
        property string headerText: ""

        sourceComponent: ContextBar {
            id: contextBarComponent
        }

        Common.Divider {
            anchors.top: parent.bottom
        }
    }

    HeaderComponent {
        id: headerBarComponent
    }

    //search component loaded to search convos and contacts
    SearchComponent {
        id: searchBarComponent
    }

    GroupFlow.NewGroupComponent {
        id: newGroupComponent
    }

    NewContactComponent {
        id: newContactComponent
    }


    Loader {
        id: convoBuilderLoader
    }

    SideBarState {
        id: sideBarState
        anchors.fill: parent
    }
}
