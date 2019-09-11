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

    ///--- Border between SearchBar and the Pane Contents (contacts)
    Common.Divider {
        id: searchBarBorder
        anchor: utilityBar.bottom
        color: QmlCfg.palette.secondaryColor
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
