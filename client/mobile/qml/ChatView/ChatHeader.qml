import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"
import "qrc:/imports/"

ToolBar {
    width: parent.width
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }
    id: chatBar

    property bool search: false
    AnimIconButton {
        id: backButton
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        anchors.bottomMargin: CmnCfg.smallMargin
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        onTapped: mainView.pop()
    }

    RowLayout {
        anchors.left: backButton.right
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.rightMargin: CmnCfg.smallMargin
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        Label {
            Layout.alignment: Qt.AlignLeft
            Layout.topMargin: CmnCfg.microMargin
            text: mainView.currentItem.headerTitle
            font.family: CmnCfg.headerFont.family
            font.pixelSize: CmnCfg.headerFontSize
            color: CmnCfg.palette.iconFill
            visible: chatBar.state !== "search"
        }

        AnimIconButton {
            id: searchButton
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/search-icon.svg"
            visible: chatBar.state !== "search"
            onTapped: chatBar.state = "search"
        }

        BorderedTextField {
            id: searchField
            visible: chatBar.state === "search"
            enabled: visible
            Layout.margins: CmnCfg.smallMargin
            Layout.topMargin: CmnCfg.microMargin
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignCenter
            onVisibleChanged: if (visible)
                                  forceActiveFocus()
            onTextEdited: {
                ownedMessages.searchPattern = text
            }
            font.pixelSize: CmnCfg.chatTextSize
        }

        AnimIconButton {
            id: searchExitButton
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/x-icon.svg"
            visible: chatBar.state === "search"
            onTapped: chatBar.state = "default"
        }
    }

    states: [
        State {
            name: "default"
            StateChangeScript {
                script: {
                    ownedMessages.searchActive = false
                    searchField.text = ""
                    // clear search
                }
            }
        },
        State {
            name: "search"
            StateChangeScript {
                script: {
                    ownedMessages.searchActive = true
                }
            }
        }
    ]
}
