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
        onClicked : {
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
            onTapped: {
                searchField.text = ''
                // TODO then focus search field again
            }
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

