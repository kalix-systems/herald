import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

//NPB: just looks kind bad
Rectangle {
    property color startColor: "light blue"

    radius: QmlCfg.radius
    color: startColor
    width: parent.width
    height: Math.max(textCol.height, 20)

    Button {
        id: exitButton
        font.pixelSize: 10

        anchors {
            margins: QmlCfg.smallMargin
            right: parent.right
            top: parent.top
        }

        background: Rectangle {
            color: Qt.darker(startColor, 1.9)
            height: 15
            width: height
            radius: height
            Text {
                anchors.centerIn: parent
                text: qsTr("x") //todo: make this an icon
            }
        }

        onClicked: {
            chatTextArea.state = "default"
            chatTextArea.replyText = ""
            chatTextArea.replyName = ""
        }
    }

    Column {
        id: textCol
        padding: QmlCfg.smallMargin
        Label {
            id: sender
            text: opName
            Layout.margins: QmlCfg.smallMargin
            Layout.bottomMargin: QmlCfg.smallMargin
            Layout.preferredHeight: QmlCfg.margin
            font.bold: true
        }

        TextEdit {
            text: opText
            Layout.leftMargin: QmlCfg.smallMargin
            Layout.rightMargin: QmlCfg.smallMargin
            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            Layout.alignment: Qt.AlignLeft
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
        }
    }

    Rectangle {
        z: -1
        color: startColor
        height: 15
        width: parent.width
        anchors.verticalCenter: parent.bottom
    }
}
