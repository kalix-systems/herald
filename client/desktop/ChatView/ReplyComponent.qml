import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "../common" as Common
import LibHerald 1.0

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
    id: wrapper
    radius: QmlCfg.radius
    color: startColor
    width: parent.width
    height: Math.max(textCol.height + QmlCfg.margin, 20)

    property color startColor
    property string opText: parent.opText
    property string opName: parent.opName

    Button {
        id: exitButton

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

            // this icon is very slightly off center everywhere it's used, i think it needs to be fixed
            Image {
                id: x
                source: "qrc:/x-icon-white.svg"
                anchors.centerIn: parent
                sourceSize: Qt.size(15, 15)
            }
            ColorOverlay {
                anchors.fill: x
                source: x
                color: QmlCfg.palette.iconMatte
            }
        }

        onClicked: chatTextArea.state = "default"
    }

    ColumnLayout {
        id: textCol

        Label {
            id: sender
            text: opName
            Layout.margins: QmlCfg.smallMargin
            Layout.bottomMargin: QmlCfg.smallMargin
            Layout.preferredHeight: QmlCfg.margin
            font.bold: true
            color: QmlCfg.palette.mainTextColor
        }

        TextMetrics {
            id: opTextMetrics
            text: opText
            elideWidth: (wrapper.width - QmlCfg.smallMargin) * 2
            elide: Text.ElideRight
        }

        TextEdit {
            text: opTextMetrics.elidedText
            Layout.maximumWidth: wrapper.width - QmlCfg.smallMargin
            Layout.margins: QmlCfg.smallMargin
            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            Layout.alignment: Qt.AlignLeft
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
            color: QmlCfg.palette.mainTextColor
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
