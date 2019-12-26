import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import "./../js/ContactView.mjs" as JS
import "../popups" as Popups
import "qrc:/imports/Entity" as Av
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
            // TODO possible to handle this case in ConversationLabel?
            labelComponent: GridLayout {
                id: labelGrid
                rows: bodyText.lineCount > 1 ? 3 : 2
                columns: 2
                width: parent.width
                Label {
                    id: uid
                    font {
                        bold: true
                        family: CmnCfg.chatFont.name
                        // TODO change when we make font defaults make sense
                        pixelSize: 14
                    }
                    // TODO negative margin--handle better in Platonic Rectangle
                    Layout.topMargin: labelGrid.rows > 2 ? -CmnCfg.smallMargin : 0
                    Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                    Layout.preferredHeight: labelGrid.height * 0.25
                    Layout.maximumWidth: parent.width
                    elide: "ElideRight"
                    text: messageData.conversationTitle
                    color: messageRectangle.state
                           !== "" ? CmnCfg.palette.black : CmnCfg.palette.lightGrey
                }

                Label {
                    id: ts
                    font {
                        family: CmnCfg.chatFont.name
                        //TODO: Magic number erasure, we need a secondary small label size
                        pixelSize: 11
                    }
                    text: Utils.friendlyTimestamp(messageData.time)
                    Layout.preferredHeight: labelGrid.height * 0.25
                    Layout.alignment: Qt.AlignRight | Qt.AlignTop
                    color: messageRectangle.state
                           !== "" ? CmnCfg.palette.offBlack : CmnCfg.palette.medGrey
                }

                TextMetrics {
                    id: prefix
                    text: messageData.beforeFirstMatch
                    elide: Text.ElideLeft
                    elideWidth: labelGrid.width * 2
                }

                Label {
                    id: bodyText
                    font {
                        family: CmnCfg.chatFont.name
                        //TODO: Magic number erasure
                        pixelSize: 13
                    }
                    // TODO negative margin--handle better in Platonic Rectangle
                    Layout.topMargin: labelGrid.rows > 2 ? -CmnCfg.smallMargin : 0
                    elide: "ElideRight"
                    text: if (messageData.beforeFirstMatch.length === 0) {
                              messageData.firstMatch + messageData.afterFirstMatch
                          } else if (prefix.length === messageData.beforeFirstMatch.length) {
                              prefix.elidedText + messageData.firstMatch
                                      + messageData.afterFirstMatch
                          } else {
                              "..." + prefix.elidedText + messageData.firstMatch
                                      + messageData.afterFirstMatch
                          }

                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignLeft | Qt.alignTop
                    Layout.maximumHeight: labelGrid.height
                    color: messageRectangle.state
                           !== "" ? CmnCfg.palette.black : CmnCfg.palette.lightGrey
                    textFormat: Text.StyledText
                    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                }
            }
        }
    }
}
