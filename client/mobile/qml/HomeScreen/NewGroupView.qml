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
                        mainView.pop()
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

    Rectangle {
        id: topRect
        anchors.top: parent.top
        height: CmnCfg.units.dp(60)
        width: parent.width
        property alias profPic: groupImageLoader.imageSource

        Rectangle {
            width: CmnCfg.units.dp(42)
            height: width
            color: CmnCfg.palette.black
            anchors.centerIn: parent
            Loader {
                id: groupImageLoader
                active: false
                z: 100
                property string imageSource
                anchors.fill: parent
                sourceComponent: Image {
                    //  source: imageSource
                    anchors.fill: parent
                    fillMode: Image.PreserveAspectCrop
                }
            }

            IconButton {
                anchors.centerIn: parent
                imageSource: "qrc:/camera-icon.svg"
                color: CmnCfg.palette.iconFill
            }
        }
    }
}
