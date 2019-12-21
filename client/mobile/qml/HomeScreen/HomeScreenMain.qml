import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
import "../Common" as Common
import QtGraphicalEffects 1.0

Page {
    id: cvMainView

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    Common.Drawer {
        id: contextDrawer
        DrawerContents {}
    }

    // the body of this entire element
    // displays conversations
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
                readonly property var conversationIdProxy: conversationId
                readonly property int colorProxy: model.color
                readonly property ConversationContent ownedConversationContent: ConversationContent {
                    conversationId: conversationIdProxy
                }
                convContent: ownedConversationContent

                colorCode: colorProxy
            }
        }
    }

    ColorOverlay {
        id: disabledOverlay
        visible: false
        anchors.fill: parent
        color: "black"
        opacity: 0.5

        TapHandler {
            //grabPermissions: PointerHandler.TakeOverForbidden
            onTapped: {
                cvMainView.state = "default"
            }
        }
    }

    Component {
        id: fab
        FloatingActionButtons {}
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
            name: "search"
            PropertyChanges {
                target: listViewLoader
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
        }
    ]
}
