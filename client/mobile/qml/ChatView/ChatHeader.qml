import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"
import "qrc:/imports/"

RowLayout {
    anchors {
        fill: parent
        rightMargin: CmnCfg.largeMargin
        leftMargin: CmnCfg.largeMargin
    }

    AnimIconButton {
        id: backButton
        Layout.alignment: Qt.AlignLeft
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        onTapped: mainView.pop()
    }

    Label {
        Layout.alignment: Qt.AlignCenter
        text: mainView.currentItem.headerTitle
        font: CmnCfg.headerFont
        color: CmnCfg.palette.iconFill
        visible: parent.state != "search"
    }

    AnimIconButton {
        id: searchButton
        Layout.alignment: Qt.AlignRight
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/search-icon.svg"
        visible: parent.state != "search"
        onTapped: parent.state = "search"
    }

    BorderedTextField {
        visible: parent.state === "search"
        Layout.margins: CmnCfg.smallMargin
        Layout.topMargin: 0
        Layout.fillWidth: true
        Layout.alignment: Qt.AlignCenter
        onVisibleChanged: if (visible)
                              forceActiveFocus()
        onTextEdited: {
            ownedMessages.searchPattern = text
        }
    }

    AnimIconButton {
        id: searchExitButton
        Layout.alignment: Qt.AlignRight
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/x-icon.svg"
        visible: parent.state === "search"
        onTapped: parent.state = "default"
    }

    states: [
        State {
            name: "default"
            StateChangeScript {
                script: {
                    ownedMessages.searchActive = false
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
