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
ColumnLayout {
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

    height: parent.height

    ///--- SearchBar for contacts, add contact button
    SBUtils.UtilityBar {
        id: utilityBar
        Layout.alignment: Qt.AlignTop
    }

    ///--- Border between SearchBar and the Pane Contents (contacts)
    Common.Divider {
        id: searchBarBorder
        Layout.alignment: Qt.AlignTop
        color: QmlCfg.palette.secondaryColor
    }


    ///--- Contacts View Actual
    Pane {
        padding: 0
        Layout.alignment: Qt.AlignTop
        Layout.fillHeight: true
        Layout.fillWidth:  true

        SBUtils.ContactView {
            id: contactsListView
            Layout.fillHeight: true
            Layout.fillWidth:  true
            model: contactsModel
        }

        SBUtils.ConversationView {
            id: conversationsListView
            Layout.fillHeight: true
            Layout.fillWidth:  true
            model: conversationsModel
        }
    }
}
