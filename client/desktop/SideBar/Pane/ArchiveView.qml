import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "qrc:/common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import "../../ChatView" as CV
import ".././js/ContactView.mjs" as JS
import "../popups" as Popups
import Qt.labs.platform 1.1

/// --- displays a list of archived conversations
ListView {
    id: conversationList
    clip: true
    currentIndex: -1

    interactive: true
    height: contentHeight

    delegate: Item {
        id: conversationItem

        readonly property var conversationData: model
        readonly property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise
        property bool outbound: convContent.messages.lastAuthor === Herald.config.configId
        property ConversationContent convContent: ConversationContent {
            conversationId: conversationIdProxy
        }

        visible: conversationData.status === 1
        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        ConversationRectangle {
            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
                acceptedButtons: Qt.RightButton
                onClicked: {
                    if (mouse.button == Qt.RightButton) {
                        convOptionsMenu.open()
                    }
                }
            }
        }

        Menu {
            id: unarchiveMenu
            MenuItem {
                text: "Unarchive conversation"
            }
        }
    }
}
