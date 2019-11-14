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
    color: CmnCfg.palette.sideBarHighlightColor
    width: parent.width
    height: Math.max(textCol.height + CmnCfg.margin, 20)

    property color startColor
    property string opText: parent.opText
    property string opName: parent.opName

    Rectangle {
        id: verticalAccent
        anchors.right: wrapper.left
        height: wrapper.height
        width: CmnCfg.smallMargin / 2
        color: startColor
    }

    Common.ButtonForm {
        id: exitButton
        anchors {
            margins: CmnCfg.smallMargin
            right: parent.right
            top: parent.top
        }
        source: "qrc:/x-icon.svg"
        scale: 0.8
        onClicked: {
            builder.clearReply()
        }

    }

    ColumnLayout {
        id: textCol

        Label {
            id: sender
            text: opName
            Layout.leftMargin: CmnCfg.smallMargin
            Layout.rightMargin: CmnCfg.smallMargin
            Layout.bottomMargin: CmnCfg.margin / 2
            Layout.topMargin: CmnCfg.margin / 2
            Layout.preferredHeight: CmnCfg.smallMargin
            font.bold: true
            color: startColor
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

}
