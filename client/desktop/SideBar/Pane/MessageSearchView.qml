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
import QtQuick.Layouts 1.3

ListView {
    id: messageSearchList

    signal messageClicked(var searchConversationId, var searchMsgId)
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

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
                onClicked: {
                    messageSearchList.messageClicked(messageData.conversation,
                                                     messageData.msgId)
                }
            }
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
                        pixelSize: 13
                    }
                    Layout.topMargin: labelGrid.rows > 2 ? -CmnCfg.smallMargin : 0
                    Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                    Layout.preferredHeight: labelGrid.height * 0.25
                    Layout.maximumWidth: parent.width
                    elide: "ElideRight"
                    text: messageData.author
                    color: CmnCfg.palette.black
                }

                Label {
                    id: ts
                    font {
                        family: CmnCfg.chatFont.name
                        pixelSize: 11
                    }
                    text: Utils.friendlyTimestamp(messageData.time)
                    Layout.preferredHeight: labelGrid.height * 0.25
                    Layout.alignment: Qt.AlignRight | Qt.AlignTop
                    color: CmnCfg.palette.offBlack
                }

                TextMetrics {
                    id: prefix
                    text: messageData.beforeFirstMatch + messageData.firstMatch
                    elide: Text.ElideLeft
                    elideWidth: labelGrid.width * 2
                }

                TextMetrics {
                    id: suffix
                    text: messageData.afterFirstMatch
                    elide: Text.ElideRight
                    elideWidth: labelGrid.width * 2
                }

                Label {
                    id: bodyText
                    font {
                        family: CmnCfg.chatFont.name
                        pixelSize: 13
                    }
                    Layout.topMargin: labelGrid.rows > 2 ? -CmnCfg.smallMargin : 0
                    elide: "ElideRight"
                    text: if (messageData.beforeFirstMatch.length === 0) {
                              messageData.firstMatch + messageData.afterFirstMatch
                          } else {
                              prefix.elidedText + suffix.elidedText
                          }
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignLeft | Qt.alignTop
                    Layout.maximumHeight: labelGrid.height
                    color: CmnCfg.palette.offBlack
                    textFormat: Text.StyledText
                    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                }
            }
        }
    }
}
