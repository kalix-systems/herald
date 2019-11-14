import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
import "../Common" as Common

Page {
    id: cvMainView

    header: Loader {
        id: headerLoader
        sourceComponent: CVHeader {}
    }

    background: Rectangle {
        color: CmnCfg.palette.mainColor
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
            model: conversationsModel
            delegate: CVListItem {
                readonly property var conversationIdProxy: conversationId
                readonly property int colorProxy: model.color
                readonly property Messages ownedMessages: Messages {
                    conversationId: conversationIdProxy
                }
                colorCode: colorProxy
            }
        }
    }

    // floating pencil button to trigger
    // new message flow
    ComposeButton {

        anchors {
            bottom: parent.bottom
            right: parent.right
            margins: CmnCfg.units.dp(12)
        }

        iconSource: "qrc:/pencil-icon-black.svg"
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
        }
    ]
}
