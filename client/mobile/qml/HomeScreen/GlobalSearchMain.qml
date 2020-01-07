import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
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

    Column {
        id: contentCol
        anchors {
            fill: parent
            leftMargin: CmnCfg.smallMargin
            rightMargin: CmnCfg.smallMargin
        }

        Text {
            text: qsTr("Conversations")
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: CmnCfg.smallMargin
            Layout.topMargin: CmnCfg.microMargin
            font: CmnCfg.sectionHeaderFont
            color: CmnCfg.palette.offBlack
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
                colorCode: model.color
                imageSource: Utils.safeStringOrDefault(model.picture, "")
                isGroup: !model.pairwise
                lastMsgDigest: model.lastMsgDigest
                isEmpty: model.isEmpty
                // TODO(cmck) avoid instantiating multiple ConversationContent
                // items per conversation (HomeScreenMain already carestes one
                // per conversation). Future refactor: create a single
                // ConversationContent for a ChatView when it's pushed
//                convoContent: ConversationContent {
//                    conversationId: model.conversationId
//                }
                visible: model.matched
                height: visible ? CmnCfg.convoHeight : 0
            }
        }

        Text {
            text: qsTr("Messages")
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: CmnCfg.smallMargin
            Layout.topMargin: CmnCfg.microMargin
            font: CmnCfg.sectionHeaderFont
            color: CmnCfg.palette.offBlack
        }

        ListView {
            id: messageSearchView
            height: contentHeight
            width: parent.width
            // conversations and messages are in a single column,
            // this needs to be uninteractive so that they scroll together
            interactive: false

            signal messageClicked(var searchConversationId, var searchMsgId)

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

                    // TODO load messages when data model is fixed
                    TapHandler {
                        onTapped: {
                            print('someday this will open a conversation')
                        }
                    }
                }
            }
        }
    }
}
