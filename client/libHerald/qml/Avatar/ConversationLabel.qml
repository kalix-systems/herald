import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import LibHerald 1.0

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

    // labeling constants
    anchors.fill: parent

    Label {
        id: uid
        anchors {
            top: parent.top
            left: parent.left
            right: ts.left
            rightMargin: QmlCfg.margin
        }
        font.bold: true
        font.pointSize: 17
        elide: "ElideRight"
        text: contactName
        color: QmlCfg.palette.mainTextColor
    }

    Label {
        id: ts
        anchors {
            bottom: uid.bottom
            right: parent.right
        }
        font.pointSize: 13
        text: lastTimestamp
        color: QmlCfg.palette.secondaryTextColor
    }

    Label {
        id: bodyText
        anchors {
            left: parent.left
            right: receiptImage.left
            bottom: parent.bottom
            rightMargin: QmlCfg.margin
        }
        font.pointSize: 15
        elide: "ElideRight"
        text: lastBody
        color: QmlCfg.palette.secondaryTextColor
    }

    Image {
        id: receiptImage
        anchors {
            bottom: parent.bottom
            right: parent.right
        }
        // in the future this should be some function call from common
        source: lastReceipt
        sourceSize: Qt.size(QmlCfg.units.dp(12), QmlCfg.units.dp(12))
        mipmap: true
        layer.enabled: true
        layer.samplerName: "maskSource"
        layer.effect: ShaderEffect {
            property color overlay: QmlCfg.palette.mainTextColor
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
