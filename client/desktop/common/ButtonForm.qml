import QtQuick 2.4
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtGraphicalEffects 1.13
import LibHerald 1.0

Button {
    property string source
    property alias scale: background.scale
    property color fill: QmlCfg.palette.iconMatte
    height: 25
    width: height

    background: Image {
        id: background
        source: parent.source
        sourceSize: Qt.size(24, 24)
        height: width
        mipmap: true
        layer.enabled: true
        layer.samplerName: "maskSource"
        layer.effect: ShaderEffect {
            property color overlay: fill
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
