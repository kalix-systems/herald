import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "../../Common"

RowLayout {
    id: chatRowLayout
    readonly property real textareaHeight: CmnCfg.units.dp(36)
    property bool send: cta.text.length > 0
    width: parent.width
    spacing: 0

    TextArea {
        id: cta
        height: textareaHeight
        Layout.fillWidth: true
        Layout.alignment: Qt.AlignBottom
        placeholderText: "Send a message..."
        wrapMode: "WrapAtWordBoundaryOrAnywhere"
        color: CmnCfg.palette.mainTextColor

        font {
            pointSize: CmnCfg.chatPreviewSize
            family: CmnCfg.chatFont.name
        }

        background: Rectangle {
            color: CmnCfg.palette.mainColor
        }
    }

    Grid {
        columns: cta.lineCount > 1 ? 1 : 2
        Layout.alignment: Qt.AlignRight | Qt.AlignBottom
        Layout.margins: CmnCfg.units.dp(12)
        Layout.bottomMargin: CmnCfg.units.dp(6)
        spacing: CmnCfg.units.dp(12)

        IconButton {
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/camera-icon.svg"
        }

        IconButton {
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.iconFill
            tapCallback: send ? function () {
                ownedMessages.builder.body = cta.text
                ownedMessages.builder.finalize()
                cta.clear()
            } : function () {}
            imageSource: send ? "qrc:/send-icon.svg" : "qrc:/plus-icon.svg"
        }
    }
}
