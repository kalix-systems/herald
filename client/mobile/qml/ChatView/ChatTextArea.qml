import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "../Common"

RowLayout {
    id: chatRowLayout
    readonly property real textareaHeight: CmnCfg.units.dp(24)

    readonly property var select: function () {
        cta.forceActiveFocus()
    }

    property bool send: cta.text.length > 0
    property string chatName: 'conversation'
    width: parent.width
    spacing: 0

    TextArea {
        id: cta
        height: chatRowLayout.textareaHeight
        placeholderText: qsTr('Message ' + chatRowLayout.chatName)
        wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
        color: CmnCfg.palette.black
        selectionColor: CmnCfg.palette.highlightColor
        font {
            pixelSize: CmnCfg.chatTextSize
            family: CmnCfg.chatFont.name
        }
        Layout.fillWidth: true
        Layout.alignment: Qt.AlignVCenter
        Keys.onPressed: {

            if ((event.key === Qt.Key_Backspace || event.key === Qt.Key_Delete)
                    && cta.text.length === 0) {
                Qt.inputMethod.hide()
            }
        }
    }

    Grid {
        // TODO: Collapse options into plus when typing
        // TODO: this is a binding loop use TextMetrics
        columns: cta.lineCount > 1 ? 1 : 2
        Layout.alignment: Qt.AlignRight | Qt.AlignBottom
        Layout.margins: CmnCfg.units.dp(12)
        Layout.bottomMargin: CmnCfg.units.dp(12)
        spacing: CmnCfg.units.dp(12)

        AnimIconButton {
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.black
            imageSource: "qrc:/camera-icon.svg"
        }

        AnimIconButton {
            Layout.alignment: Qt.AlignRight
            color: CmnCfg.palette.black
            tapCallback: send ? function () {
                ownedMessages.builder.body = cta.text
                ownedMessages.builder.finalize()
                cta.clear()
            } : function () {}
            imageSource: send ? "qrc:/send-icon.svg" : "qrc:/plus-icon.svg"
        }
    }
}
