import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"

RowLayout {

    anchors {
        fill: parent
        rightMargin: CmnCfg.margin
        leftMargin: CmnCfg.margin
    }

    IconButton {
        id: backButton
        Layout.alignment: Qt.AlignLeft
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        tapCallback: function () {
            mainView.pop()
        }
    }

    Row {
        Layout.alignment: Qt.AlignCenter

        Label {
            text: title
            font {
                pixelSize: CmnCfg.headerTextSize
                family: CmnCfg.labelFont.name
                bold: true
            }

            anchors.verticalCenter: parent.verticalCenter
            color: CmnCfg.palette.iconFill
        }
    }

    IconButton {
        id: searchButton
        Layout.alignment: Qt.AlignRight
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/search-icon.svg"
    }
}
