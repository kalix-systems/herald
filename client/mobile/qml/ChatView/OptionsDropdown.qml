import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble"
import "../Common" as CMN

Rectangle {
    property var cb
    signal deactivate
    signal activate
    width: parent.width
    visible: height != 0
    color: CmnCfg.palette.offBlack
    onActivate: height = 50
    onDeactivate: height = 0
    height: 0
    property bool active: height > 0

    Behavior on height {
        NumberAnimation {
            easing.type: Easing.InOutQuad
            duration: 100
        }
    }

    Connections {
        target: chatList
        onCloseDropdown: {
            if (active)
                deactivate()
        }
    }

    Row {
        anchors.fill: parent
        anchors.rightMargin: CmnCfg.defaultMargin
        clip: true
        layoutDirection: Qt.RightToLeft
        spacing: CmnCfg.defaultMargin

        CMN.AnimIconButton {
            icon.color: CmnCfg.palette.white
            onTapped: deactivate()
            imageSource: "qrc:/x-icon.svg"
            anchors.verticalCenter: parent.verticalCenter
        }

        CMN.AnimIconButton {
            imageSource: "qrc:/info-icon.svg"
            anchors.verticalCenter: parent.verticalCenter
            visible: !bubbleLoader.isAux
            icon.color: CmnCfg.palette.white
            onTapped: {
                mainView.push(cb.infoPage)
                deactivate()
            }
        }

        CMN.AnimIconButton {
            imageSource: "qrc:/emoticon-icon.svg"
            icon.color: CmnCfg.palette.white
            anchors.verticalCenter: parent.verticalCenter
        }
        CMN.AnimIconButton {
            icon.color: CmnCfg.palette.white
            imageSource: "qrc:/reply-icon.svg"
            anchors.verticalCenter: parent.verticalCenter
            onTapped: {
                ownedMessages.builder.opId = msgId
                deactivate()
            }
        }
    }
}
