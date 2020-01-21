import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"
import "qrc:/imports/"

ToolBar {
    property alias searchText: searchField.text
    property alias searchPlaceholderText: searchField.placeholderText

    property var parentPage

    width: parent.width
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    AnimIconButton {
        id: backButton
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        anchors.bottomMargin: CmnCfg.smallMargin
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        onTapped: {
            // Search state is cleared on back button press, instead of on
            // destruction of this component, to preserve search query text
            // when this page is an entry in the app StackView (e.g. if a user
            // clicks on a conversation item and then presses the back button
            // to return to the search view)
            Herald.users.clearFilter()
            Herald.conversations.clearFilter()
            Herald.messageSearch.clearSearch()
            mainView.pop(StackView.Immediate)
        }
    }

    RowLayout {
        id: searchRow
        anchors.left: backButton.right
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        height: implicitHeight

        BorderedTextField {
            id: searchField
            color: CmnCfg.palette.white
            borderColor: "transparent"
            placeholderText: parentPage.state
                             === "fromComposeButton" ? qsTr("Enter username or group title") : qsTr(
                                                           "Search your conversations")
            // Load previous search query in search field in case user gets
            // to this view via back button and expects state to be preserved
            text: Herald.conversations.filter
            font.pixelSize: CmnCfg.chatTextSize
            font.family: CmnCfg.defaultFont.family

            Layout.margins: CmnCfg.smallMargin
            Layout.topMargin: CmnCfg.microMargin
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignCenter
            Component.onCompleted: forceActiveFocus()

            Keys.onPressed: {
                // this makes sure that returns and tabs are not evaluated
                if (event.key === Qt.Key_Return || event.key === Qt.Key_Tab) {
                    event.accepted = true
                }
            }

            onTextChanged: {
                Qt.callLater(function (text) {
                    Herald.conversations.filter = text
                    Herald.messageSearch.searchPattern = text
                }, searchField.text)
            }
        }

        AnimIconButton {
            id: clearButton
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/x-icon.svg"
            visible: searchField.text === "" ? false : true
            onTapped: searchField.text = ""
            // TODO then focus search field again
        }
    }

    Rectangle {
        height: 1
        color: CmnCfg.palette.lightGrey
        anchors {
            bottomMargin: CmnCfg.smallMargin
            bottom: parent.bottom
            left: searchRow.left
            leftMargin: CmnCfg.microMargin
            right: searchRow.right
        }
    }
}
