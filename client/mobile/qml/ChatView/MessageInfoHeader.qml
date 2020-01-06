import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"

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

    Item {
        Layout.fillWidth: true
    }

    Label {
        text: "More Info"
        font: CmnCfg.headerFont
        color: CmnCfg.palette.iconFill
        Layout.alignment: Qt.AlignCenter
    }

    Item {
        Layout.fillWidth: true
    }
}
