import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "./../js/ContactView.mjs" as JS
import "../popups" as Popups
import "qrc:/imports/Avatar" as Av
import "../../ChatView" as CV

ListView {
    id: messageSearchList
    clip: true
    currentIndex: -1
    height: contentHeight
    interactive: false

    delegate: Item {
        id: messageItem
        property var messageData: model
        property bool outbound: messageData.author === herald.config.configId

        height: CmnCfg.convoHeight
        width: parent.width

        Common.PlatonicRectangle {
            id: messageRectangle
            boxColor: messageData.conversationColor
            boxTitle: messageData.conversationTitle
            picture: Utils.safeStringOrDefault(messageData.conversationPicture,
                                               "")
            groupPicture: !messageData.conversationPairwise
            labelComponent: Av.ConversationLabel {
                contactName: messageData.conversationTitle
                labelColor: CmnCfg.palette.secondaryColor
                labelSize: 14
                lastAuthor: outbound ? "You" : messageData.author
                lastBody: lastAuthor + ": " + messageData.body
                lastTimestamp: Utils.friendlyTimestamp(messageData.time)
            }
        }
    }
}
