import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import QtQuick 2.12
import LibHerald 1.0
import "../ChatView" as ChatView
import "../Common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/Entity" as Entity

Page {
    id: searchView
    readonly property Component headerComponent: GlobalSearchHeader {}
    property Loader headerLoader

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    Component.onCompleted: appRoot.router.searchView = searchView
    signal messageClicked(var searchConversationId, var searchMsgId)
    signal convoClicked(var searchConversationId)

    Column {
        id: contentCol
        anchors {
            fill: parent
        }

        Text {
            text: qsTr("Conversations")
            anchors {
                left: parent.left
                leftMargin: CmnCfg.smallMargin
                topMargin: CmnCfg.smallMargin
            }

            font.family: CmnCfg.labelFont.name
            font.weight: Font.DemiBold
            font.pixelSize: CmnCfg.labelFontSize
            color: CmnCfg.palette.offBlack
            bottomPadding: 0
        }

        ListView {
            height: contentHeight
            width: parent.width
            // conversations and messages are in a single column,
            // this needs to be uninteractive so that they scroll together
            interactive: false

            model: Herald.conversations
            delegate: ConversationItem {
                itemTitle: title
                colorCode: model.conversationColor
                imageSource: Utils.safeStringOrDefault(model.picture, "")
                isGroup: !model.pairwise
                lastMsgDigest: model.lastMsgDigest
                isEmpty: model.isEmpty
                tapEnabled: false
                visible: model.matched
                height: visible ? CmnCfg.convoHeight : 0
                TapHandler {
                    onTapped: convoClicked(model.conversationId)
                }
            }
        }

        Text {
            text: qsTr("Messages")
            anchors {
                left: parent.left
                leftMargin: CmnCfg.smallMargin
                topMargin: CmnCfg.smallMargin
            }
            bottomPadding: 0
            font.family: CmnCfg.labelFont.name
            font.weight: Font.DemiBold
            font.pixelSize: CmnCfg.labelFontSize
            color: CmnCfg.palette.offBlack
        }

        ListView {
            id: messageSearchView
            height: contentHeight
            width: parent.width
            // conversations and messages are in a single column,
            // this needs to be uninteractive so that they scroll together
            interactive: false

            model: Herald.messageSearch
            delegate: Item {
                height: CmnCfg.convoHeight
                width: parent.width

                Common.PlatonicRectangle {
                    id: messageRectangle
                    boxTitle: model.conversationTitle
                    boxColor: model.conversationColor
                    picture: Utils.safeStringOrDefault(
                                 model.conversationPicture, "")
                    isGroupPicture: !model.conversationPairwise
                    isMessageResult: true

                    labelComponent: Entity.MessageSearchLabel {
                        conversationTitle: model.conversationTitle
                        timestamp: Utils.friendlyTimestamp(model.time)
                        labelColor: CmnCfg.palette.black
                        secondaryLabelColor: CmnCfg.palette.offBlack

                        beforeMatch: model.beforeFirstMatch
                        match: model.firstMatch
                        afterMatch: model.afterFirstMatch
                    }

                    TapHandler {
                        onTapped: {
                            messageClicked(model.conversation, model.msgId)
                        }
                    }
                }
            }
        }
    }
}
