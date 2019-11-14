import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common

Component {
// this uses a rectangle and anchors instead of a layout because that manages the
// spacing behaviour better. (there is no change in layout on resize, anchors more correct)
Rectangle {
    anchors.fill: parent
    Rectangle  {
        id: topRect
        anchors.top: parent.top
        height: 70
        width: parent.width
        color: "transparent"

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
        id: titleText
        anchors.top: topRect.bottom
        leftPadding: 12
        placeholderText: "Group title"
    }

    Rectangle {
        anchors.top: titleText.bottom
        id: divider
        height: 2
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "black"
    }

    Rectangle {
        anchors.top: divider.bottom
        anchors.topMargin: 20
        id: bigDivider
        height: 1
        width: parent.width
        color: CmnCfg.palette.secondaryTextColor
    }


    TextArea {
        id: groupSelectText
        anchors.top: bigDivider.bottom
        anchors.topMargin: 20
        leftPadding: 12
        placeholderText: "Add members"
    }

    Rectangle {
        anchors.top: groupSelectText.bottom
        height: 2
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "black"
    }
}
}
