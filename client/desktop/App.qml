import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import "./SideBar"
import "../foundation/js/utils.mjs" as Utils

Item {
    id: appRoot

    // TODO this can be passed as an argument wherever it's needed
    // PAUL 0: this can be passed as an argument to a C++ helper function.
    // currently the issue is with scoping, instead we can just pass
    // this index like an argument with a dynamic property in C++
    property int gsSelectedIndex: -1

    anchors.fill: parent.fill

    TopMenuBar {
        Popups.ConfigPopup {
            id: preferencesPopup
        }
    }

    Users {
        id: contactsModel
    }

    Conversations {
        id: conversationsModel
    }

    Popups.ColorPicker {
        id: avatarColorPicker
    }

    Popups.ConfigPopup {
        id: configPopup
    }

    Config {
        id: config
    }

    // PAUL: it is time for real art here.
    Component {
        id: splash
        Image {
            anchors.fill: parent
            source: "qrc:/land.png"
            mipmap: true
        }
    }

    SplitView {
        id: rootSplitView
        anchors.fill: parent
        orientation: Qt.Horizontal

        SideBarMain {
            id: sideBar
        }

        Loader {
            id: chatView
            sourceComponent: splash
        }

        handle: Item {Rectangle {
            id: handle
            implicitWidth: 1
            color: CmnCfg.palette.borderColor
        }
            Rectangle {

            }
        }
    }

    Component.onCompleted: networkHandle.login()
}
