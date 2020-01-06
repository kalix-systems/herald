import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"
import "qrc:/imports/"

RowLayout {
    property alias searchText: searchField.text

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
        onTapped: {
            // Search state is cleared on back button press, instead of on
            // destruction of this component, to preserve search query text
            // when this page is an entry in the app StackView (e.g. if a user
            // clicks on a conversation item and then presses the back button
            // to return to the search view)
            Herald.users.clearFilter()
            Herald.conversations.clearFilter()
            Herald.messageSearch.clearSearch()
            mainView.pop()
        }
    }

    Item {
        Layout.fillWidth: true
        Layout.leftMargin: CmnCfg.defaultMargin
        Layout.alignment: Qt.AlignTop
        height: parent.height - CmnCfg.microMargin

        BorderedTextField {
            id: searchField
            color: CmnCfg.palette.white
            borderColor: "Transparent"
            placeholderText: qsTr('Search your conversations')
            font.pixelSize: CmnCfg.units.dp(18)
            // Load previous search query in search field in case returns gets
            // to this view via back button and expects state to be preserved
            text: Herald.conversations.filter

            Component.onCompleted: forceActiveFocus()

            Keys.onPressed: {
                // this makes sure that returns and tabs are not evaluated
                if (event.key === Qt.Key_Return
                        || event.key === Qt.Key_Tab) {
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
            anchors {
                right: parent.right
                bottom: parent.bottom
                bottomMargin: CmnCfg.microMargin
            }
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/x-icon.svg"
            onTapped: searchField.text = ''
            // TODO then focus search field again
        }

        Rectangle {
            height: 1
            color: CmnCfg.palette.lightGrey
            anchors {
                bottom: parent.bottom
                left: parent.left
                right: parent.right
            }
        }
    }
}
