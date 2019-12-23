import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick 2.12

Column {
    width: CmnCfg.units.dp(48)
    spacing: CmnCfg.units.dp(16)

    Button {
        id: button
        height: CmnCfg.units.dp(48)
        width: height

        Label {
            id: label
            anchors.right: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: CmnCfg.units.dp(16)
            text: qsTr("New contact")
            font.pointSize: 12
            padding: CmnCfg.units.dp(4)
            background: Rectangle {
                anchors.fill: parent
                color: CmnCfg.palette.lightGrey
            }
        }

        onClicked: {
            mainView.push(newContactViewMain)
            cvMainView.state = "default"
        }
        icon.source: "qrc:/add-contact-icon.svg"
        icon.color: CmnCfg.palette.black
        icon.height: CmnCfg.units.dp(36)
        icon.width: CmnCfg.units.dp(36)
        anchors.horizontalCenter: parent.horizontalCenter

        background: Rectangle {
            color: parent.pressed ? Qt.darker(CmnCfg.palette.lightGrey,
                                              1.3) : CmnCfg.palette.lightGrey
            anchors.fill: parent
            radius: height
        }
    }

    Button {
        height: CmnCfg.units.dp(48)
        width: height
        Label {
            anchors.right: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: CmnCfg.units.dp(16)
            text: qsTr("New group")
            font.pointSize: 12
            padding: CmnCfg.units.dp(4)
            background: Rectangle {
                anchors.fill: parent
                anchors.margins: -CmnCfg.units.dp(3)
                color: CmnCfg.palette.lightGrey
            }
        }

        icon.source: "qrc:/contacts-icon.svg"
        icon.color: CmnCfg.palette.black
        icon.height: CmnCfg.units.dp(36)
        icon.width: CmnCfg.units.dp(36)
        anchors.horizontalCenter: parent.horizontalCenter

        background: Rectangle {
            color: parent.pressed ? Qt.darker(CmnCfg.palette.lightGrey,
                                              1.3) : CmnCfg.palette.lightGrey
            anchors.fill: parent
            radius: height
        }

        TapHandler {
            onTapped: {
                mainView.push(newGroupViewMain)
                cvMainView.state = "default"
            }
        }
    }

    Button {

        Label {
            anchors.right: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: CmnCfg.units.dp(16)
            text: qsTr("New conversation")
            font.pointSize: 12
            padding: CmnCfg.units.dp(4)
            background: Rectangle {
                anchors.fill: parent
                color: CmnCfg.palette.lightGrey
            }
        }

        height: CmnCfg.units.dp(54)
        width: height

        icon.source: "qrc:/pencil-icon-black.svg"
        icon.color: CmnCfg.palette.black
        icon.height: CmnCfg.units.dp(48)
        icon.width: CmnCfg.units.dp(48)

        background: Rectangle {
            color: parent.pressed ? Qt.darker(CmnCfg.palette.lightGrey,
                                              1.3) : CmnCfg.palette.lightGrey
            anchors.fill: parent
            radius: height
        }
    }
}
