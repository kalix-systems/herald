import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import "qrc:/common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils

Common.PlatonicRectangle {
    id: convoRectangle
    boxTitle: title
    boxColor: conversationData.color
    picture: Utils.safeStringOrDefault(conversationData.picture, "")
    isGroupPicture: !conversationData.pairwise
    labelComponent: Av.ConversationLabel {
        contactName: title
        lastBody: !convContent.messages.isEmpty ? lastAuthor + ": "
                                                  + convContent.messages.lastBody : ""
        lastAuthor: outbound ? qsTr("You") : convContent.messages.lastAuthor
        lastTimestamp: !convContent.messages.isEmpty ? Utils.friendlyTimestamp(
                                                           convContent.messages.lastTime) : ""
        labelColor: convoRectangle.state !== "" ? CmnCfg.palette.black : CmnCfg.palette.lightGrey
        secondaryLabelColor: convoRectangle.state
                             !== "" ? CmnCfg.palette.offBlack : CmnCfg.palette.medGrey
        labelFontSize: CmnCfg.entityLabelSize
    }
}
