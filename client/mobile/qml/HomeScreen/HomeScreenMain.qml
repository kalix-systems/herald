import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
import "../Headers" as Headers
import "../Common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import QtGraphicalEffects 1.0

// The home page of the entire application
// contains a list of conversations by default
Page {
    id: cvMainView
    readonly property Component headerComponent: Headers.HomeHeader {}
    background: Rectangle {
        color: CmnCfg.palette.white
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
                itemTitle: title
                colorCode: model.color
                imageSource: Utils.safeStringOrDefault(model.picture, "")
                isGroup: !model.pairwise
                convContent: ConversationContent {
                    conversationId: model.conversationId
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
            //grabPermissions: PointerHandler.TakeOverForbidden
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
                onTapped: {
                    cvMainView.state = "fabButtonState"
                    buttonLoader.sourceComponent = fab
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
        }
    ]
}
