import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "SideBar" as SBUtils

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
    property alias contactsListView : contactsListView
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

    ///--- SearchBar for contacts, add contact button
    SBUtils.UtilityBar {
        id: utilityBar
        anchors.top: toolBar.bottom
    }


    // FC: WE use this border pattern in a few places and it is redundant DRY
    ///--- Border between SearchBar and the Pane Contents (contacts)
    Rectangle {
        id: searchBarBorder
        anchors.top: utilityBar.bottom
        color: QmlCfg.palette.secondaryColor
        width: parent.width
        height: 1
    }

    ///--- Contacts View Actual
    Pane {
        padding: 0
        anchors {
            right: parent.right
            left: parent.left
            top: searchBarBorder.bottom
            bottom: parent.bottom
        }

        SBUtils.ContactView {
            id: contactsListView
            anchors.fill: parent
            model: contactsModel
        }
    }
}

/*##^## Designer {
    D{i:10;anchors_height:1}
}
 ##^##*/

