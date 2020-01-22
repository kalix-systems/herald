import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick 2.12

Column {
    width: CmnCfg.fabDiameter
    spacing: CmnCfg.largeMargin

    Button {
        height: CmnCfg.miniFabDiameter
        width: height

        Label {
            anchors.right: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: CmnCfg.defaultMargin
                                 + (CmnCfg.fabDiameter - CmnCfg.miniFabDiameter) / 2
            text: qsTr("New contact")
            font.pointSize: 12
            padding: CmnCfg.smallMargin
            background: Rectangle {
                anchors.fill: parent
                color: CmnCfg.palette.lightGrey
            }
        }

        TapHandler {
            gesturePolicy: TapHandler.ReleaseWithinBounds
            onTapped: {
                mainView.push(newContactViewMain)
                cvMainView.state = "default"
            }
        }

        icon.source: "qrc:/add-contact-icon.svg"
        icon.color: CmnCfg.palette.black
        icon.height: CmnCfg.iconSize
        icon.width: CmnCfg.iconSize
        anchors.horizontalCenter: parent.horizontalCenter

        background: Rectangle {
            color: parent.pressed ? Qt.darker(CmnCfg.palette.lightGrey,
                                              1.3) : CmnCfg.palette.lightGrey
            anchors.fill: parent
            radius: height
        }
    }

    Button {
        height: CmnCfg.miniFabDiameter
        width: height

        Label {
            anchors.right: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: CmnCfg.defaultMargin
                                 + (CmnCfg.fabDiameter - CmnCfg.miniFabDiameter) / 2
            text: qsTr("New group")
            font.pointSize: 12
            padding: CmnCfg.smallMargin
            background: Rectangle {
                anchors.fill: parent
                anchors.margins: -CmnCfg.units.dp(3)
                color: CmnCfg.palette.lightGrey
            }
        }

        icon.source: "qrc:/contacts-icon.svg"
        icon.color: CmnCfg.palette.black
        icon.height: CmnCfg.iconSize
        icon.width: CmnCfg.iconSize
        anchors.horizontalCenter: parent.horizontalCenter

        background: Rectangle {
            color: parent.pressed ? Qt.darker(CmnCfg.palette.lightGrey,
                                              1.3) : CmnCfg.palette.lightGrey
            anchors.fill: parent
            radius: height
        }

        TapHandler {
            gesturePolicy: TapHandler.ReleaseWithinBounds
            onTapped: {
                mainView.push(newGroupViewMain)
                cvMainView.state = "default"
            }
        }
    }

    Button {
        height: CmnCfg.fabDiameter
        width: height

        Label {
            anchors.right: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: CmnCfg.defaultMargin
            text: qsTr("Send a message")
            font.pointSize: 12
            padding: CmnCfg.smallMargin
            background: Rectangle {
                anchors.fill: parent
                color: CmnCfg.palette.lightGrey
            }
        }

        icon.source: "qrc:/pencil-icon.svg"
        icon.color: CmnCfg.palette.black
        icon.height: CmnCfg.iconSize
        icon.width: CmnCfg.iconSize

        background: Rectangle {
            color: parent.pressed ? Qt.darker(CmnCfg.palette.lightGrey,
                                              1.3) : CmnCfg.palette.lightGrey
            anchors.fill: parent
            radius: height
        }

        TapHandler {
            gesturePolicy: TapHandler.ReleaseWithinBounds
            onTapped: {
                cvMainView.state = "default"
                mainView.push(globalSearchView, {
                                  "state": "fromComposeButton"
                              }, StackView.Immediate)
            }
        }
    }
}
