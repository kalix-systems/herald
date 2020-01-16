import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../Common"
import "qrc:/imports/"

ToolBar {
    width: parent.width
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }
    property bool searchState: false

    AnimIconButton {
        id: backButton
        color: CmnCfg.palette.iconFill
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        anchors.bottomMargin: CmnCfg.smallMargin
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
            text: qsTr("Contacts")
            font.family: CmnCfg.headerFont.family
            font.pixelSize: CmnCfg.headerFontSize
            color: CmnCfg.palette.iconFill
            visible: !searchState
        }

        AnimIconButton {
            id: searchButton
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/search-icon.svg"
            visible: !searchState
            onTapped: searchState = true
        }

        BorderedTextField {
            visible: searchState
            enabled: visible
            Layout.margins: CmnCfg.smallMargin
            Layout.topMargin: CmnCfg.microMargin
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignCenter
            onVisibleChanged: if (visible)
                                  forceActiveFocus()
            placeholderText: "Search contacts"
            onTextEdited: {
                Herald.users.filter = text
            }
            font.pixelSize: CmnCfg.chatTextSize
        }

        AnimIconButton {
            id: searchExitButton
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/x-icon.svg"
            visible: searchState
            onTapped: {
                searchState = false
                Herald.users.filter = ""
            }
        }
    }
}
