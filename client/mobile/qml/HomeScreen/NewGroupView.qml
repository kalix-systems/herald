import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
import "../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils
import "GroupFlowComponents"

Page {
    id: newGroupView
    height: mainView.height
    header: ToolBar {
        id: conversationViewHeader

        clip: true
        height: CmnCfg.toolbarHeight

        background: Rectangle {
            color: CmnCfg.palette.offBlack
        }

        RowLayout {
            anchors.fill: parent
            Row {
                Layout.alignment: Qt.AlignLeft
                Layout.leftMargin: CmnCfg.units.dp(12)
                spacing: CmnCfg.units.dp(16)
                IconButton {
                    id: backButton
                    color: CmnCfg.palette.iconFill
                    imageSource: "qrc:/back-arrow-icon.svg"
                    tapCallback: function () {
                        mainView.pop(null)
                    }
                }

                Label {
                    id: stateLabel
                    text: qsTr("New group")
                    font {
                        pixelSize: CmnCfg.chatPreviewSize
                        family: CmnCfg.labelFont.name
                    }
                    anchors.verticalCenter: parent.verticalCenter
                    color: CmnCfg.palette.iconFill
                }
            }
        }
    }

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    GroupHeaderComponent {
        id: topRect
    }

    Rectangle {
        anchors.top: topRect.bottom
        id: bigDivider
        height: 1
        width: parent.width
        color: CmnCfg.palette.black
    }

    ContactsSearchComponent {
        id: groupSelectText
    }

    Button {
        anchors.top: groupSelectText.bottom
        anchors.topMargin: CmnCfg.defaultMargin / 2
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.units.dp(28)

        width: CmnCfg.units.dp(60)
        height: CmnCfg.units.dp(30)

        background: Rectangle {
            anchors.fill: parent
            color: CmnCfg.palette.offBlack
        }

        Text {
            text: qsTr("CREATE")
            anchors.centerIn: parent
            color: CmnCfg.palette.white
            font.family: CmnCfg.labelFont.name
        }
        TapHandler {
            onTapped: {
                if (topRect.groupTitle === "") {
                    Herald.conversationBuilder.setTitle(qsTr("Untitled Group"))
                } else {
                    Herald.conversationBuilder.setTitle(topRect.groupTitle)
                }

                //TODO: impl for setting prof pic once file dialog exists
                //                if (topRect.profPic !== "") {
                //                }
                Herald.conversationBuilder.finalize()
                mainView.pop()
            }
        }
    }
}
