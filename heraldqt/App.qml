import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import QtQml 2.13

Item {
    id: appRoot

    property var gsConversationId
    property int  gsSelectedIndex: -1
    property color gsConvoColor
    property bool gsContactsSearch: true // contacts search vs conversations search

    anchors.fill: parent.fill
    Layout.fillWidth: true
    Layout.fillHeight: true

    NetworkHandle {
        id: networkHandle
        onNewMessageChanged: messageModel.refresh()
        onNewContactChanged: contactsModel.refresh()
    }

    Messages {
        id: messageModel
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
                contactsModel.setColor(gsSelectedIndex, avatarColorPicker.colorIndex)
                appRoot.gsConvoColor = QmlCfg.avatarColors[avatarColorPicker.colorIndex]
                avatarColorPicker.close()
            }
        }
    }


    Config {
        id: config
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

        ChatView {
            id: chatView
        }

        handle: Rectangle {
            implicitWidth: 3
            color: QmlCfg.palette.secondaryColor
        }
    }
}
