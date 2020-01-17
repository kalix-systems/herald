import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "qrc:/common" as Common
import "qrc:/imports/" as Imports

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

    maximumFlickVelocity: 1500
    flickDeceleration: sideBarPaneRoot.height * 10
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

        // Convo search list items for adding new groups/contacts directly from
        // the search results view
        Repeater {
            model: ListModel {
                ListElement {
                    iconSource: "qrc:/contacts-icon.svg"
                    label: qsTr("Create new group")
                    newState: "newGroupState"
                }

                ListElement {
                    iconSource: "qrc:/add-contact-icon.svg"
                    label: qsTr("Message new contact")
                    newState: "newContactState"
                }
            }

            Rectangle {
                visible: sideBarState.state === "globalSearch"
                height: CmnCfg.avatarSize
                width: parent.width
                color: hoverHandler.containsMouse ?
                           CmnCfg.palette.lightGrey : "transparent"

                Imports.IconButton {
                    id: createGroupIcon
                    icon.source: model.iconSource
                    icon.color: hoverHandler.containsMouse ?
                                    CmnCfg.palette.black :
                                    CmnCfg.palette.iconFill
                    anchors {
                        left: parent.left
                        leftMargin: CmnCfg.smallMargin +
                                    (CmnCfg.avatarSize - CmnCfg.iconSize) / 2
                        verticalCenter: parent.verticalCenter
                    }

                    padding: 0
                    background: Item {}
                }

                Label {
                    text: model.label
                    font.family: CmnCfg.chatFont.name
                    font.pixelSize: CmnCfg.entityLabelSize
                    font.weight: Font.Medium
                    color: hoverHandler.containsMouse ?
                               CmnCfg.palette.black :
                               CmnCfg.palette.lightGrey
                    anchors {
                        left: createGroupIcon.right
                        leftMargin: (CmnCfg.avatarSize - CmnCfg.iconSize) / 2 +
                                    CmnCfg.defaultMargin
                        verticalCenter: parent.verticalCenter
                    }
                }

                MouseArea {
                    id: hoverHandler
                    z: CmnCfg.overlayZ
                    hoverEnabled: true
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: sideBarState.state = model.newState
                }
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
