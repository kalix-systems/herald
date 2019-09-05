import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import Qt.labs.platform 1.0
import QtQml 2.13

ApplicationWindow {
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    id: root
    minimumWidth: 250
    minimumHeight: 300

    MenuBar {
        Menu {
            title: "Herald"
            MenuItem {
                text: "Preferences"
                onTriggered: preferencesPopup.open()
            }
        }
        Menu {
            title: "Window"
            MenuItem {
                text: "Minimize"
                onTriggered: root.showMinimized()
            }
        }
    }

    NetworkHandle {
        id: networkHandle
    }

    Popups.ConfigPopup {
        id: preferencesPopup
    }

    /// global configurations item
    Config {
        id: config
        Component.onCompleted: {
            if (!config.exists()) {
                preferencesPopup.open()
                print("placeholder for a popup which forces first time config.")
            }
        }
    }

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
            id: sideBar
        }

        /// placeholder element
        ChatView {
            id: chatView
        }

        handle: Rectangle {
            implicitWidth: 4
            implicitHeight: 4
            color: if (SplitHandle.pressed) {
                       Qt.darker(QmlCfg.palette.secondaryColor, 1.1)
                   } else {
                       QmlCfg.palette.secondaryColor
                   }
        }
    }
}
