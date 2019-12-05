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
    Rectangle {
        id: topRect
        anchors.top: parent.top
        height: CmnCfg.units.dp(72)
        width: mainView.width
        property alias profPic: groupImageLoader.imageSource

        Rectangle {
            id: cameraSection
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

                tapCallback: function () {
                    print("TODO implement group pics")
                }
            }
        }
        Rectangle {
            anchors.topMargin: CmnCfg.units.dp(24)
            anchors.top: cameraSection.bottom
            width: parent.width - CmnCfg.units.dp(56)
            height: CmnCfg.units.dp(72)
            anchors.horizontalCenter: parent.horizontalCenter
            TextArea {
                id: titleText
                anchors.top: parent.top
                anchors.left: parent.left
                placeholderText: "Group title"
                leftPadding: 0
            }

            Rectangle {
                anchors.bottom: titleText.bottom
                id: divider
                height: 1
                width: parent.width
                color: "black"
            }
        }
    }
}
