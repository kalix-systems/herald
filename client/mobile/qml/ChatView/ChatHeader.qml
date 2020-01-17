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
        id: searchRow
        anchors.left: backButton.right
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.rightMargin: CmnCfg.smallMargin
        anchors.right: searchExitButton.left
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

        BorderedTextField {
            id: searchField
            visible: chatBar.state === "search"
            enabled: visible
            borderColor: "transparent"
            Layout.margins: CmnCfg.smallMargin
            Layout.topMargin: CmnCfg.microMargin
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignCenter
            onVisibleChanged: if (visible)
                                  forceActiveFocus()
            onTextEdited: {
                ownedMessages.searchPattern = text

                const x = chatList.scrollBar.position
                const y = chatList.scrollBar.size
                //TODO: why doesn't this work?
                ownedMessages.setSearchHint(x, y)
                if (ownedMessages.searchNumMatches > 0) {
                    chatList.positionViewAtIndex(ownedMessages.prevSearchMatch(
                                                     ), ListView.Center)
                }
            }
            font.pixelSize: CmnCfg.chatTextSize
        }
        AnimIconButton {
            id: back
            imageSource: "qrc:/up-chevron-icon.svg"
            color: CmnCfg.palette.lightGrey
            Layout.alignment: Qt.AlignVCenter
            enabled: (chatBar.state === "search"
                      && ownedMessages.searchNumMatches > 0)
            visible: chatBar.state === "search"
            opacity: enabled ? 1 : 0.5
            onTapped: chatList.positionViewAtIndex(
                          ownedMessages.prevSearchMatch(), ListView.Center)
        }

        AnimIconButton {
            id: forward
            imageSource: "qrc:/down-chevron-icon.svg"
            color: CmnCfg.palette.lightGrey
            Layout.alignment: Qt.AlignVCenter
            enabled: (chatBar.state === "search"
                      && ownedMessages.searchNumMatches > 0)
            visible: chatBar.state === "search"
            opacity: enabled ? 1 : 0.5

            onTapped: chatList.positionViewAtIndex(
                          ownedMessages.nextSearchMatch(), ListView.Center)
        }
    }

    AnimIconButton {
        id: searchButton
        anchors.verticalCenter: parent.verticalCenter
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.smallMargin
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/search-icon.svg"
        visible: chatBar.state !== "search"
        onTapped: chatBar.state = "search"
    }
    AnimIconButton {
        id: searchExitButton
        anchors.verticalCenter: parent.verticalCenter
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.smallMargin
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/x-icon.svg"
        visible: chatBar.state === "search"
        onTapped: chatBar.state = "default"
    }
    Rectangle {
        height: 1
        color: CmnCfg.palette.lightGrey
        visible: chatBar.state === "search"
        anchors {
            bottomMargin: CmnCfg.smallMargin + 1
            bottom: parent.bottom
            left: searchRow.left
            leftMargin: CmnCfg.microMargin
            right: searchRow.right
        }
    }

    Component.onDestruction: {
        ownedMessages.searchActive = false
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
