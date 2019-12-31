import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "qrc:/common" as Common

Flickable {
    id: sideBarPaneRoot

    property alias messageSearchLoader: messageSearchLoader
    property alias sideBarBodyLoader: sideBarBodyLoader
    property alias sideBarFlowLoader: sideBarFlowLoader

    anchors.fill: parent
    interactive: true
    contentHeight: wrapperCol.height
    boundsBehavior: Flickable.StopAtBounds

    ScrollBar.vertical: ScrollBar {
        policy: ScrollBar.AsNeeded
        width: CmnCfg.smallMargin
    }

    //column to load content, components are inside instead of being declared separately because
    // otherwise loader cannot keep track of contentHeight of the listviews.
    Column {
        id: wrapperCol
        width: parent.width

        Text {
            text: qsTr("Conversations")
            anchors.left: parent.left
            anchors.leftMargin: CmnCfg.smallMargin
            topPadding: CmnCfg.smallMargin
            font.bold: true
            font.family: CmnCfg.chatFont.name
            color: CmnCfg.palette.lightGrey
            visible: sideBarState.state === "globalSearch"
        }

        Loader {
            id: sideBarBodyLoader
            property bool archiveState: false
            sourceComponent: Component {
                ConversationViewMain {
                    id: convosLvComponent
                    model: Herald.conversations
                    state: sideBarBodyLoader.archiveState ? "archivestate" : ""
                }
            }
            width: parent.width

            Loader {
                id: sideBarFlowLoader
                anchors.fill: active ? parent : undefined
                active: false
                z: active ? parent.z + 1 : -1
            }
        }

        Text {
            text: qsTr("Messages")
            anchors.left: parent.left
            anchors.leftMargin: CmnCfg.smallMargin
            topPadding: CmnCfg.smallMargin
            font.bold: true
            font.family: CmnCfg.chatFont.name
            color: CmnCfg.palette.lightGrey
            visible: sideBarState.state === "globalSearch"
        }

        Loader {
            id: messageSearchLoader
            width: parent.width
            property var searchModel

            //model loaded into search view only in search state
            sourceComponent: Component {
                MessageSearchView {
                    model: searchModel
                }
            }
        }
    }
}
