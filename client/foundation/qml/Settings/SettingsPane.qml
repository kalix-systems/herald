import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports"
import "./SettingsComponents" as SetsComps

Flickable {
    property alias mainColumn: col
    Layout.fillWidth: true
    Layout.fillHeight: true
    contentHeight: col.height
    boundsBehavior: Flickable.StopAtBounds

    property var cropCallbackArg

    Column {
        id: col
        spacing: CmnCfg.smallMargin
        topPadding: CmnCfg.smallMargin
        anchors.right: parent.right
        anchors.left: parent.left
        SetsComps.SettingsListItem {
            id: profile
            headerText: qsTr("Profile information")
            settingsContent: SetsComps.Profile {}
        }
        SetsComps.SettingsListItem {
            id: notifications
            headerText: qsTr("Notifications")
            settingsContent: SetsComps.Notifications {}
        }
        SetsComps.SettingsListItem {
            id: appearance
            headerText: qsTr("Appearance")
            settingsContent: SetsComps.Appearance {}
        }
        SetsComps.SettingsListItem {
            id: security
            headerText: qsTr("Privacy & Security")
            settingsContent: SetsComps.Privacy {}
        }

        SetsComps.SettingsListItem {
            id: storage
            headerText: qsTr("Data & Storage")
            settingsContent: SetsComps.Storage {}
        }

        SetsComps.SettingsListItem {
            id: advanced
            headerText: qsTr("Advanced")
            settingsContent: SetsComps.Advanced {}
        }

        SetsComps.SettingsListItem {
            id: feedback
            headerText: qsTr("Help & Feedback")
            settingsContent: SetsComps.Feedback {}
        }
    }
}
