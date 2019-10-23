import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "qrc:/common" as Common
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
    radius: CmnCfg.radius
    color: startColor
    width: parent.width
    height: Math.max(textCol.height + CmnCfg.margin, 20)

    property color startColor
    property string opText: parent.opText
    property string opName: parent.opName

    Button {
        id: exitButton

        anchors {
            margins: CmnCfg.smallMargin
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
                color: CmnCfg.palette.iconMatte
            }
        }

        onClicked: {
            builder.clearReply()
            //chatTextArea.state = "default"
        }
    }

    ColumnLayout {
        id: textCol

        Label {
            id: sender
            text: opName
            Layout.margins: CmnCfg.smallMargin
            Layout.bottomMargin: CmnCfg.smallMargin
            Layout.preferredHeight: CmnCfg.margin
            font.bold: true
            color: CmnCfg.palette.mainTextColor
        }

        TextMetrics {
            id: opTextMetrics
            text: opText
            elideWidth: (wrapper.width - CmnCfg.smallMargin) * 2
            elide: Text.ElideRight
        }

        TextEdit {
            text: opTextMetrics.elidedText
            Layout.maximumWidth: wrapper.width - CmnCfg.smallMargin
            Layout.margins: CmnCfg.smallMargin
            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            Layout.alignment: Qt.AlignLeft
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
            color: CmnCfg.palette.mainTextColor
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
