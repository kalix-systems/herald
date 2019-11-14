import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common

Component {
Column {
    anchors.fill: parent

    Rectangle  {
        height: 70
        width: parent.width
        color: CmnCfg.palette.paneColor

        Rectangle {
            anchors.top: parent.top
            anchors.topMargin: CmnCfg.largeMargin
            anchors.horizontalCenter: parent.horizontalCenter
            width: 42
            height: width
            color: "black"

            Common.ButtonForm {
                anchors.centerIn: parent
                source: "qrc:/camera-icon.svg"
                fill: CmnCfg.palette.paneColor
            }
        }
    }

    TextArea {
        leftPadding: 12
        placeholderText: "Group title"
    }

    Rectangle {
        height: 2
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "black"
    }
    Item {
        height: 20
        width: parent.width
    }

    Rectangle {
        height: 1
        width: parent.width
        color: CmnCfg.palette.secondaryTextColor
    }

    Item {
        height: 20
        width: parent.width
    }

    TextArea {
        leftPadding: 12
        placeholderText: "Add members"
    }

    Rectangle {
        height: 2
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "black"
    }

    Item {
        height: 20
        width: parent.width
    }

}
}
