import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "qrc:/common" as Common

Flickable {
    id: sideBarPaneRoot

    property alias messageSearchLoader: messageSearchLoader
    property alias sideBarBodyLoader: sideBarBodyLoader

    anchors.fill: parent
    interactive: true
    contentHeight: wrapperCol.height
    boundsBehavior: Flickable.StopAtBounds

    ScrollBar.vertical: ScrollBar {
        policy: ScrollBar.AsNeeded
        width: CmnCfg.padding
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
            color: CmnCfg.palette.lightGrey
            visible: sideBarState.state === "globalSearch"
        }

        Loader {
            id: sideBarBodyLoader
            sourceComponent: Component {
                ConversationViewMain {
                    id: convosLvComponent
                    model: Herald.conversations
                }
            }
            width: parent.width
        }

        Text {
            text: qsTr("Messages")
            anchors.left: parent.left
            anchors.leftMargin: CmnCfg.smallMargin
            topPadding: CmnCfg.smallMargin
            font.bold: true
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
