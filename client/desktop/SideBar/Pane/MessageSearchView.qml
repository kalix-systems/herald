import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "./../js/ContactView.mjs" as JS
import "../popups" as Popups
import "qrc:/imports/Entity" as Ent
import "../../ChatView" as CV
import QtQuick.Layouts 1.3

ListView {
    id: messageSearchList

    signal messageClicked(var searchConversationId, var searchMsgId)
    currentIndex: -1
    height: contentHeight
    // conversations and messages are in a single column,
    // this needs to be uninteractive so that they scroll together
    interactive: false

    delegate: Item {
        id: messageItem
        property var messageData: model
        property bool outbound: messageData.author === Herald.config.configId

        height: CmnCfg.convoHeight
        width: parent.width

        Common.PlatonicRectangle {
            id: messageRectangle
            boxColor: messageData.conversationColor
            boxTitle: messageData.conversationTitle
            picture: Utils.safeStringOrDefault(messageData.conversationPicture,
                                               "")
            isGroupPicture: !messageData.conversationPairwise
            isMessageResult: true

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent

                onClicked: messageSearchList.messageClicked(
                               messageData.conversation, messageData.msgId)
            }

            // TODO spacing is awkward
            labelComponent: Ent.MessageSearchLabel {
                conversationTitle: messageData.conversationTitle
                timestamp: Utils.friendlyTimestamp(messageData.time)
                labelColor: messageRectangle.state !== "" ? CmnCfg.palette.black : CmnCfg.palette.lightGrey
                secondaryLabelColor: messageRectangle.state
                                     !== "" ? CmnCfg.palette.offBlack : CmnCfg.palette.medGrey
                labelFontSize: CmnCfg.entityLabelSize

                beforeMatch: messageData.beforeFirstMatch
                match: messageData.firstMatch
                afterMatch: messageData.afterFirstMatch
            }
        }
    }
}
