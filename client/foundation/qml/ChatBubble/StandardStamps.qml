import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

RowLayout {
    Layout.margins: CmnCfg.smallMargin / 2
    Label {
        font.pixelSize: 10
        text: friendlyTimestamp
        id: timestamp
        color: CmnCfg.palette.mainTextColor
    }

    Item {
        Layout.fillWidth: true
    }

    Image {
        id: receipt
        source: receiptImage
        sourceSize: Qt.size(12, 12)
        layer.enabled: true
        layer.samplerName: "maskSource"
        layer.effect: ShaderEffect {
            property color overlay: CmnCfg.palette.iconMatte
            property var source: background
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
