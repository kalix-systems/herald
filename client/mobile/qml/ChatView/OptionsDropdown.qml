import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble"
import "../Common" as CMN

Rectangle {
    property ChatBubble cb
    signal deactivate
    signal activate
    width: parent.width
    visible: height != 0
    color: CmnCfg.palette.lightGrey
    onActivate: height = 50
    onDeactivate: height = 0

    Behavior on height {
        NumberAnimation {
            easing.type: Easing.InOutQuad
            duration: 100
        }
    }

    RowLayout {
        anchors.fill: parent
        clip: true

        Item {
            Layout.preferredWidth: parent.width / 2
        }

        CMN.AnimIconButton {
            Layout.alignment: Qt.AlignRight
            iconSize: Qt.size(CmnCfg.units.dp(24), CmnCfg.units.dp(24))
            imageSource: "qrc:/reply-icon.svg"
            onTapped: {
                ownedMessages.builder.opId = msgId
                deactivate()
            }
        }
        CMN.AnimIconButton {
            Layout.alignment: Qt.AlignRight
            iconSize: Qt.size(CmnCfg.units.dp(24), CmnCfg.units.dp(24))

            imageSource: "qrc:/lenny-icon.svg"
        }
        CMN.AnimIconButton {
            Layout.alignment: Qt.AlignRight
            iconSize: Qt.size(CmnCfg.units.dp(24), CmnCfg.units.dp(24))

            imageSource: "qrc:/options-icon.svg"
        }
        CMN.AnimIconButton {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: CmnCfg.largeMargin
            iconSize: Qt.size(CmnCfg.units.dp(24), CmnCfg.units.dp(24))
            onClicked: deactivate()
            imageSource: "qrc:/x-icon.svg"
        }
    }
}
