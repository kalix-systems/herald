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
        onTapped: {
            if (chatBar.state === "search")
                chatBar.state = "default"
            else
                mainView.pop()
        }
    }

    RowLayout {
        id: searchRow
        anchors {
            left: backButton.right
            leftMargin: CmnCfg.smallMargin
            right: searchButtons.left

            rightMargin: CmnCfg.defaultMargin
            verticalCenter: parent.verticalCenter
        }

        GridLayout {
            Layout.alignment: Qt.AlignLeft
            Layout.topMargin: CmnCfg.microMargin
            visible: chatBar.state !== "search"
            Label {
                elide: Label.ElideRight
                text: mainView.currentItem.headerTitle
                font.family: CmnCfg.headerFont.family
                font.pixelSize: CmnCfg.headerFontSize
                color: CmnCfg.palette.iconFill
                Layout.maximumWidth: searchRow.width
            }
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
            placeholderText: qsTr("Search conversation")
            onVisibleChanged: if (visible)
                                  forceActiveFocus()
            onTextEdited: {
                ownedMessages.searchPattern = text

                const x = chatList.scrollBar.position
                const y = chatList.scrollBar.size
                ownedMessages.setSearchHint(x, y)
                if (ownedMessages.searchNumMatches > 0) {
                    chatList.positionViewAtIndex(ownedMessages.prevSearchMatch(
                                                     ), ListView.Center)
                }
            }
            font.pixelSize: CmnCfg.chatTextSize
        }

        Label {
            property int searchIndex: ownedMessages.searchNumMatches
                                      > 0 ? ownedMessages.searchIndex : 0
            property int searchNum: ownedMessages.searchNumMatches
                                    > 0 ? ownedMessages.searchNumMatches : 0
            bottomPadding: CmnCfg.units.dp(2)
            text: searchIndex + "/" + searchNum
            Layout.alignment: Qt.AlignVCenter
            visible: (chatBar.state === "search"
                      && ownedMessages.searchNumMatches > 0)
            font: CmnCfg.defaultFont
            color: CmnCfg.palette.white
            verticalAlignment: TextInput.AlignBottom
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

        AnimIconButton {
            id: clearButton
            imageSource: "qrc:/x-icon.svg"
            iconSize: CmnCfg.units.dp(20)
            color: CmnCfg.palette.iconFill
            Layout.alignment: Qt.AlignVCenter
            visible: (chatBar.state === "search" && searchField.text !== "")
            onTapped: searchField.text = ""
        }
    }

    Row {
        id: searchButtons
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        anchors.rightMargin: CmnCfg.defaultMargin
        spacing: CmnCfg.defaultMargin
        layoutDirection: Qt.RightToLeft
        visible: chatBar.state !== "search"
        width: visible ? implicitWidth : 0

        AnimIconButton {
            id: timerButton
            anchors.verticalCenter: parent.verticalCenter
            imageSource: timerMenu.chosenTimer
            color: "transparent"
            topPadding: 1
            onTapped: timerMenu.open()
        }

        AnimIconButton {
            id: searchButton
            anchors.verticalCenter: parent.verticalCenter
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/search-icon.svg"
            onTapped: chatBar.state = "search"
        }
    }

    //    AnimIconButton {
    //        id: clearButton
    //        anchors.verticalCenter: parent.verticalCenter
    //        anchors.right: parent.right
    //        anchors.rightMargin: CmnCfg.smallMargin
    //        color: CmnCfg.palette.iconFill
    //        imageSource: "qrc:/x-icon.svg"
    //        visible: chatBar.state === "search"
    //        onTapped: chatBar.state = "default"
    //    }
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

    TimerOptions {
        id: timerMenu
        conversationItem: chatPage.convoItem
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
