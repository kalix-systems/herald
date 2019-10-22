import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "../../Common"

RowLayout {
    readonly property real textareaHeight: CmnCfg.units.dp(36)
    TextArea {
        id: cta
        Layout.fillWidth: true
        height: textareaHeight
        font {
            pointSize: CmnCfg.chatPreviewSize
            family: CmnCfg.chatFont.name
        }
        color: CmnCfg.palette.mainTextColor
        background: Rectangle {
            anchors.fill: parent
            color: CmnCfg.palette.secondaryColor
        }
    }

    Loader {
        Layout.alignment: Qt.AlignRight | Qt.AlignBottom
        sourceComponent: cta.text.length > 0 ? sendButton : atcButton
    }

    Component {
        id: atcButton
        IconButton {}
    }

    Component {
        id: sendButton
        Button {
            height: textareaHeight
        }
    }
}
