import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
import "../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0

Page {
    id: newGroupView
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
                    text: "New group"
                    font {
                        pointSize: CmnCfg.chatPreviewSize
                        family: CmnCfg.chatFont.name
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
        color: "black"
    }
}
