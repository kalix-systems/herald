import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "NewConvoComponents" as ConvUtils
import "qrc:/common" as Common

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
Pane {
    id: contactPane
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
        border.color: CmnCfg.palette.secondaryColor
        color: CmnCfg.palette.mainColor
    }

    ///--- SearchBar for contacts, add contact button
    Column {
        id: contextBar
        width: parent.width
        Loader {
            property string searchPlaceholder: ""
            property bool contactsSearch: false
            id: searchLoader
            sourceComponent: contextBarComponent
            width: parent.width
        }
    }

    ContextBar {
        id: contextBarComponent
    }

    //search component loaded to search convos and contacts
    SearchComponent {
        id: searchBarComponent
    }

    //component loaded when selecting a new group
    ConvUtils.GroupSelectComponent {
        id: groupSelectComponent
    }

    //component loaded when finalizing new group
    ConvUtils.FinalizeGroupComponent {
        id: finalizeGroupComponent
    }

    ///--- Border between SearchBar and the Pane Contents (contacts)
    Common.Divider {
        id: searchBarBorder
        anchors.top: contextBar.bottom
        color: CmnCfg.palette.borderColor
    }

    ConvUtils.NewGroupBar {
        id: newGroupBar
        anchors.top: searchBarBorder.bottom
        visible: sideBarState.state === "newConversationState"
    }

    Loader {
        id: convoBuilderLoader
    }

    SideBarState {
        id: sideBarState
    }
}
