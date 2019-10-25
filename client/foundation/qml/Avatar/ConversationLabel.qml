import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as JS

// TODO:
// there are some loose magic numbers
// hanging around in the font sizes. move those to CmnCfg
// TODO:
// move the property translation functions into
// some common js directory , receipt urls are not numbers, nor are timestamps
Item {
    // the group name or displayName of the conversation
    property string contactName
    // the previous message of the conversation, or the empty string
    property string lastBody
    // the previous latest human readable timestamp, or the empty string
    property string lastTimestamp
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: 0
    property string lastAuthor
    property color labelColor
    property int labelSize
    // labeling constants
    anchors.fill: parent

    GridLayout {
        id: labelGrid
        rows: 2
        columns: 2
        columnSpacing: 6
        anchors.fill: parent

        Label {
            id: uid
            font {
                bold: true
                family: CmnCfg.chatFont.name
                pixelSize: labelSize
            }
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Layout.preferredHeight: labelGrid.height / 4
            elide: "ElideRight"
            text: contactName
            color: labelColor
        }

        Label {
            id: ts
            font {
                family: CmnCfg.chatFont.name
                pixelSize: 11
            }
            text: lastTimestamp
            Layout.preferredHeight: labelGrid.height / 4
            Layout.alignment: Qt.AlignRight | Qt.AlignTop
            color: CmnCfg.palette.secondaryColor
        }

        Label {
            id: bodyText
            font {
                family: CmnCfg.chatFont.name
                pixelSize: 13
            }
            elide: "ElideRight"
            text: lastBody
            Layout.alignment: Qt.AlignLeft | Qt.alignTop
            color: CmnCfg.palette.secondaryColor
        }

        Image {
            id: receiptImage
            // in the future this should be some function call from common
            source: JS.receiptStatusSwitch(lastReceipt)
            sourceSize: Qt.size(CmnCfg.units.dp(16), CmnCfg.units.dp(16))
            mipmap: true
            layer.enabled: true
            layer.samplerName: "maskSource"
            layer.effect: ShaderEffect {
                property color overlay: CmnCfg.palette.mainTextColor
                property var source: receiptImage
                fragmentShader: "
uniform lowp sampler2D source;
uniform lowp sampler2D maskSource;
uniform vec4 overlay;
varying highp vec2 qt_TexCoord0;
void main() {
lowp vec4 tex = texture2D(source, qt_TexCoord0);
lowp vec4 mask = texture2D(maskSource, qt_TexCoord0);
gl_FragColor = overlay * mask.a;
}
"
            }
        }
    }
}
