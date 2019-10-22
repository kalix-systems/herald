import QtQuick 2.13
import QtQuick.Controls 2.13

Column {
    id: teamColumn
    // the width of the this column
    readonly property real columnWidth: 100
    // the width of the icon at the top of the column
    readonly property real embellishmentHeight: 80
    // distance between icons in the list
    readonly property real teamIconSpacing: 80
    // the color of team icons, anything raised
    readonly property color primaryColor: "white"
    // the color of embellishments
    readonly property color secondaryColor: "black"
    // tertiary colors, mostly the bar behind the embelishment
    readonly property color tertiaryColor: "grey"
    // the model containing data displayed in the column
    readonly property ListModel teamModel: dummyList
    // the width of the margin, should be CmnCfg.margin
    readonly property real marginWidth: 10
    // the size of team icons
    readonly property real teamIconsSize: 50

    // Collapse Me
    ListModel {
        id: dummyList
        ListElement {
            team: "dummy"
            iconUrl: ""
            newMessage: false
        }
        ListElement {
            team: "dummy"
            iconUrl: ""
            newMessage: true
        }
    }

    anchors {
        left: parent.left
    }

    width: columnWidth

    // dummy height, set anchors
    height: 500

    Rectangle {
        id: background
        anchors.fill: parent
        color: primaryColor
    }

    Item {
        id: embellishment

        height: embellishmentHeight
        anchors {
            right: parent.right
            left: parent.left
            topMargin: teamIconSpacing
        }
        Rectangle {
            id: backBanner
            color: secondaryColor
            anchors.verticalCenter: parent.verticalCenter
            height: parent.height / 2
            width: parent.width
        }
        Rectangle {
            color: tertiaryColor
            anchors.verticalCenter: parent.verticalCenter
            anchors.horizontalCenter: backBanner.horizontalCenter
            height: teamIconsSize
            width: teamIconsSize
            Image {
                id: teamIcon
                source: "qrc:/chats-icon.svg"
                anchors.fill: parent
                anchors.margins: marginWidth
                sourceSize: Qt.size(48, 48)
            }
        }
    }

    Item {
        id: horizontalRule
        height: 30
        width: parent.width
        anchors.top: embellishment.bottom
        Rectangle {
            width: backBanner.width * 0.50
            height: 2
            color: "black"
            anchors.centerIn: parent
        }
    }

    ListView {
        id: teamListView
        clip: true

        anchors.top: horizontalRule.bottom
        anchors.bottom: newTeamButton.top
        spacing: teamIconSpacing
        width: parent.width

        model: dummyList
        delegate: Item {
            anchors.horizontalCenter: parent.horizontalCenter
            Rectangle {
                id: teamAvatar
                height: teamIconsSize
                width: teamIconsSize
                anchors.horizontalCenter: parent.horizontalCenter
                color: tertiaryColor
                Text {
                    color: secondaryColor
                    text: "TEAM"
                    anchors.centerIn: parent
                }
            }

            Rectangle {
                id: newMessageIndicator
                visible: newMessage
                anchors {
                    verticalCenter: teamAvatar.verticalCenter
                    horizontalCenter: teamAvatar.horizontalCenter
                    horizontalCenterOffset: -teamListView.width * 0.5
                }
                color: "#4e9cdf"
                height: teamIconsSize * 0.5
                width: newMessageIndicator.height
                radius: newMessageIndicator.height
            }
        }
    }

    Button {
        id: newTeamButton
        anchors {
            bottom: teamColumn.bottom
            horizontalCenter: parent.horizontalCenter
            bottomMargin: marginWidth
        }
        height: teamIconsSize
        width: teamIconsSize
        background: Rectangle {
            color: primaryColor
            border.color: secondaryColor
            border.width: 3
            Image {
                id: plusButton
                source: "qrc:/plus-icon.svg"
                sourceSize: Qt.size(48, 48)
                anchors.fill: parent
                anchors.margins: marginWidth
            }
        }
    }
}
