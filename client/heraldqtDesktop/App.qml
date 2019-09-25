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
        onNewMessageChanged: convModel.refresh()
        onNewContactChanged: contactsModel.refresh()
        onNewConversationChanged: conversationsModel.hardRefresh()
    }

    Conversation {
        id: convModel
    }

    Users {
        id: contactsModel
    }

    Conversations {
        id: conversationsModel
    }

    Users {
        id: conversationMembers
        conversationId: Utils.unwrapOr(convModel.conversationId, "")
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
                        implicitWidth: 1
                        color: "black"
                    }
    }
}
