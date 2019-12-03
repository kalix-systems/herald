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
            labelComponent:    GridLayout {
                id: labelGrid
                rows: 3
                columns: 2
                width: parent.width
                height: parent.height
                Label {
                    id: uid
                    font {
                        bold: true
                        family: CmnCfg.chatFont.name
                        pixelSize: 13
                    }
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

                Label {
                    id: bodyText
                    font {
                        family: CmnCfg.chatFont.name
                        pixelSize: 13
                    }
                    elide: "ElideRight"
                    text: messageData.firstMatch + messageData.afterFirstMatch
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignLeft | Qt.alignTop
                    Layout.maximumHeight: labelGrid.height * 0.75
                    color: CmnCfg.palette.offBlack
                    textFormat: Text.RichText
                    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                }

        }
    }
}
}
