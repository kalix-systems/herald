import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import QtQuick.Dialogs 1.3
import QtMultimedia 5.13

Component {
// this uses a rectangle and anchors instead of a layout because that manages the
// spacing behaviour better. (there is no change in layout on resize, anchors more correct)
Rectangle {
    anchors.fill: parent

    GroupHeaderComponent {
        id: topRect
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


    ContactSearchComponent {
        id: groupSelectText
        anchors.top: bigDivider.bottom
        anchors.topMargin: 20
    }

}
}
