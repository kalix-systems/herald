import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import QtQuick.Dialogs 1.3
import QtTest 1.12
import LibHerald 1.0
import "SideBar" as SBUtils

Pane {
    id: contactPane

    property real windowFraction: 0.25 // By default set the width to 1/4 the total window size.
    property bool isContactOnlyView: false // set true if the only view is the contact list
    property real maxWindowFraction: 0.66
    // maximum width, where root is ApplicationWindow
    SplitView.maximumWidth: root.width * maxWindowFraction
    SplitView.minimumWidth: 250
    SplitView.preferredWidth: root.width * windowFraction

    onWidthChanged: {
        windowFraction = width / root.width
    }

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
        anchors.top: toolBar.bottom
        id: utilityBar
    }

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
            anchors.fill: parent
            model: Contacts {
                id: contacts
            }
        }
    }
}

/*##^## Designer {
    D{i:10;anchors_height:1}
}
 ##^##*/

