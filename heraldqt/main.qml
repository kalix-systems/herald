import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.13
import LibHerald 1.0
import "common"


ApplicationWindow {
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    id: root
    /// global configurations item
    Config {
        id: config
        Component.onCompleted: {
            if (config.exists) {
                print("IT existed, y'all good")
            } else {
                print("placeholder for a popup which forces first time config.")
            }
        }
    }

    CommonConfig { id: comCfg }

    /// Todo : make the split handle less intrusive. probably just a line
    SplitView {

        id: rootSplitView
        anchors.fill: parent
        Layout.fillWidth: true
        Layout.fillHeight: true
        orientation: Qt.Horizontal
        /// Contacts view for the desktop client, in DesktopContacts.qml
        /// includes the config and contacts toolbars
        SideBar {
            id: contacts
        }

        /// placeholder element
        ChatView {
            id: chatView
        }
    }
}
