import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import Qt.labs.platform 1.1

Menu {
    id: messageOptionsMenu
    MenuItem {
        text: qsTr("Delete Message")
        onTriggered: ownedConversation.deleteMessage(index)
    }
    MenuItem {
        text: qsTr("More Info") + "..."
        onTriggered: {
            moreInfo.convoMembers = conversationMembers
            moreInfo.messageData = messageModelData
            moreInfo.open()
        }
    }
}
