import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import "common/utils.mjs" as Utils
import QtQml 2.13

Item {
    id: appRoot

    // TODO this can be passed as an argument wherever it's needed
    property int gsSelectedIndex: -1
    // TODO why does this need to be global?
    property var gsConvoItemMembers

    anchors.fill: parent.fill
    Layout.fillWidth: true
    Layout.fillHeight: true

    NetworkHandle {
        id: networkHandle
        // every conversation has it's own refresh signal. guards
        //        onNewMessageChanged: convModel.refresh()
        onNewContactChanged: contactsModel.refresh()
        onNewConversationChanged: conversationsModel.hardRefresh()
    }

    Users {
        id: contactsModel
    }

    Conversations {
        id: conversationsModel
    }

    Popups.ConfigPopup {
        id: preferencesPopup
    }

    Popups.ColorPicker {
        id: avatarColorPicker

        // button is here to know index of contact clicked
        Button {
            id: colorSubmissionButton
            text: "Submit"
            anchors {
                right: parent.right
                bottom: parent.bottom
            }

            onClicked: {
                contactsModel.setColor(gsSelectedIndex,
                                       avatarColorPicker.colorIndex)
                avatarColorPicker.close()
            }
        }
    }

    Config {
        id: config
    }

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
        Layout.fillWidth: true
        Layout.fillHeight: true
        orientation: Qt.Horizontal

        SideBar {
            id: sideBar
        }

        Loader {
            id: chatView
            sourceComponent: splash
        }

        handle: Rectangle {
            implicitWidth: 1.1
            color: "black"
        }
    }
}
