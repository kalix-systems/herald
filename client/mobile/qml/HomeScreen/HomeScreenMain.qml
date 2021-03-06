import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls" as Controls
import "../Common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import QtGraphicalEffects 1.0

// The home page of the entire application
// contains a list of conversations by default
Page {
    id: cvMainView
    readonly property Component headerComponent: HomeHeader {
        parentPage: cvMainView
    }

    background: Rectangle {
        color: CmnCfg.palette.white
    }
    Component.onCompleted: appRoot.router.cvView = cvMainView
    signal messagePositionRequested(var requestMsgId)
    // Used to close any open per-convo options menu bars when new one opened
    signal closeAllOptionsBars

    Label {
        id: noConvosLabel
        anchors {
            top: parent.top
            topMargin: CmnCfg.defaultMargin
            horizontalCenter: parent.horizontalCenter
            //left: parent.left
        }
        text: "No conversations to show"
        font.family: CmnCfg.chatFont.name
        font.pixelSize: CmnCfg.chatTextSize
        font.italic: true
        horizontalAlignment: Text.AlighHCenter
        visible: false
    }

    // the body of this entire element
    // displays conversations
    // TODO: figure out why this is in a loader.
    Loader {
        id: listViewLoader
        anchors.fill: parent
        sourceComponent: ListView {
            id: cvListView
            clip: true
            boundsBehavior: ListView.StopAtBounds
            anchors.fill: parent
            model: Herald.conversations
            delegate: ConversationItem {
                id: conversationItem
                property var conversationData: model
                convoContent: ContentMap.get(model.conversationId)
                isNTS: {
                    Herald.utils.compareByteArray(
                                Herald.config.ntsConversationId,
                                model.conversationId)
                }
                itemTitle: !isNTS ? convoContent.title : qsTr("Note to Self")
                colorCode: !isNTS ? convoContent.conversationColor : UserMap.get(
                                        Herald.config.configId).userColor
                imageSource: !isNTS ? Utils.safeStringOrDefault(
                                          convoContent.picture,
                                          "") : Utils.safeStringOrDefault(
                                          Herald.config.profilePicture, "")
                isGroup: !convoContent.pairwise
                lastMsgDigest: convoContent.lastMsgDigest
                isEmpty: lastMsgDigest === ""
                isArchived: convoContent.status === 1
                visible: (cvMainView.state === "archiveState" && isArchived)
                         || (cvMainView.state !== "archiveState" && !isArchived)
            }
            Connections {
                target: appRoot.router
                onConvoRequest: {
                    mainView.push(chatViewMain, {
                                      "convId": searchConversationId
                                  })
                    messagePositionRequested(searchMsgId)
                }
            }

            Connections {
                target: appRouter
                onConvoClicked: {
                    mainView.push(chatViewMain, {
                                      "convId": searchConversationId
                                  })
                }
            }

            Connections {
                target: appRouter
                onGroupRequested: {
                    mainView.push(chatViewMain, {
                                      "convId": groupId
                                  })
                }
            }
        }
    }

    // TODO: maybe make this a popup?
    ColorOverlay {
        id: disabledOverlay
        visible: false
        anchors.fill: parent
        color: "black"
        opacity: 0.5

        TapHandler {
            gesturePolicy: TapHandler.ReleaseWithinBounds
            onTapped: cvMainView.state = "default"
        }
    }

    Component {
        id: fab
        Controls.ExpandedComposeButtons {}
    }

    Component {
        id: plusButton
        Controls.ComposeButton {
            iconSource: "qrc:/plus-icon.svg"
            TapHandler {
                gesturePolicy: TapHandler.ReleaseWithinBounds
                onTapped: {
                    cvMainView.state = "fabButtonState"
                }
            }
        }
    }

    Loader {
        id: buttonLoader
        anchors {
            bottom: parent.bottom
            right: parent.right
            margins: CmnCfg.units.dp(12)
        }
        sourceComponent: plusButton
    }

    states: [
        State {
            name: "default"
            PropertyChanges {
                target: buttonLoader
                visible: true
            }
        },
        State {
            name: "fabButtonState"
            PropertyChanges {
                target: disabledOverlay
                visible: true
            }
            PropertyChanges {
                target: buttonLoader
                sourceComponent: fab
            }
        },
        State {
            name: "archiveState"
            PropertyChanges {
                target: buttonLoader
                visible: false
            }
            PropertyChanges {
                target: noConvosLabel
                visible: true
            }
        }
    ]
}
