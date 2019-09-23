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

    property var gsConversationId
    property int gsSelectedIndex: -1
    property color gsConvoColor
    property var gsConvoItemMembers
    property bool gsContactsSearch: true

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

    Users {
        id: conversationMembers
        conversationId: Utils.unwrapOr(gsConversationId, "")
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
                appRoot.gsConvoColor = QmlCfg.avatarColors[avatarColorPicker.colorIndex]
                avatarColorPicker.close()
            }
        }
    }

    Config {
        id: config
    }



    StackView {
        // ToDo: rename to root stackView
        id: rootSplitView
        anchors.fill: parent

        Component {
            id: contacts
            // ToDo: rename to contacts view
            SideBar {

            }
        }

        Component.onCompleted: {
           rootSplitView.push(contacts)
        }
    }
}
