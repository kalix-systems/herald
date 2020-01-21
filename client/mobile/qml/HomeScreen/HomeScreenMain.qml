import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
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
                isNTS: {
                    Herald.utils.compareByteArray(
                                Herald.config.ntsConversationId,
                                model.conversationId)
                }
                itemTitle: !isNTS ? title : qsTr("Note to Self")
                colorCode: !isNTS ? model.conversationColor : Herald.config.configColor
                imageSource: !isNTS ? Utils.safeStringOrDefault(
                                          model.picture,
                                          "") : Utils.safeStringOrDefault(
                                          Herald.config.profilePicture, "")
                isGroup: !model.pairwise
                lastMsgDigest: model.lastMsgDigest
                isEmpty: model.isEmpty
                convoContent: ContentMap.get(model.conversationId)
                visible: (cvMainView.state === "archiveState"
                          && model.status === 1)
                         || (cvMainView.state !== "archiveState"
                             && model.status === 0)
            }
            Connections {
                target: appRoot.router
                onConvoRequest: {
                    const conv_idx = Herald.conversations.indexById(
                                       searchConversationId)

                    // early return on out of bounds
                    if ((conv_idx < 0) || (conv_idx >= cvListView.count))
                        return

                    stackView.push(cvListView.itemAtIndex(conv_idx).ownedCV)
                    messagePositionRequested(searchMsgId)
                }
            }

            Connections {
                target: appRouter
                onConvoClicked: {

                    const conv_idx = Herald.conversations.indexById(
                                       searchConversationId)

                    // early return on out of bounds
                    if ((conv_idx < 0) || (conv_idx >= cvListView.count))
                        return

                    stackView.push(cvListView.itemAtIndex(conv_idx).ownedCV)
                }
            }
            Connections {
                target: appRouter
                onGroupRequested: {

                    const conv_idx = Herald.conversations.indexById(groupId)

                    // early return on out of bounds
                    if ((conv_idx < 0) || (conv_idx >= cvListView.count))
                        return

                    stackView.pop(null, StackView.Immediate)
                    stackView.push(cvListView.itemAtIndex(conv_idx).ownedCV)
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
        ExpandedComposeButtons {}
    }

    Component {
        id: plusButton
        ComposeButton {
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
        }
    ]
}
