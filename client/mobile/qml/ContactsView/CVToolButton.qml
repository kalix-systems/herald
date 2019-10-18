import QtQuick.Controls 2.12
import QtQuick 2.12
import LibHerald 1.0

ToolButton {

    property var tapCallback
    property string imageSource: ""
    property color color: QmlCfg.palette.iconMatte

    TapHandler {
        onTapped: tapCallback()
    }

    background: Image {
        id: icon

        anchors.fill: parent
        sourceSize: Qt.size(QmlCfg.iconSizes.medium, QmlCfg.iconSizes.medium)
        mipmap: true
        layer.enabled: true
        layer.samplerName: "maskSource"
        layer.effect: ShaderEffect {
            property color overlay: color
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
}"
        }
    }
}
