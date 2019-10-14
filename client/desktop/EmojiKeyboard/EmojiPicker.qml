import QtQuick 2.13
import QtQuick.Controls 1.0
import QtGraphicalEffects 1.12

//PAUL: demagic all numbers and colors
Item {
    id: maskShape
    readonly property color pickerColor: "light gray"
    readonly property int edgeRadius: 10
    readonly property color pickerClickColor: Qt.lighter("light gray", 1.5)
    property string modifier: ""
    property var caratCenter
    property var window
    signal send(string emoji)
    signal close

    height: 350
    width: 250

    MaskedBlur {
        id: blur
        radius: 32
        anchors.fill: mask
        source: mask
        samples: 32
        maskSource: selectorMask
    }

    // the masked version of the element
    OpacityMask {
        id: mask
        anchors.fill: selectorMask
        source: ShaderEffectSource {
            width: window.width
            height: window.height
            sourceItem: window
            sourceRect: Qt.rect(maskShape.parent.x, maskShape.parent.y - 40,
                                maskShape.width, maskShape.height)
        }
        maskSource: selectorMask
        visible: false
    }

    // the shape, used to mask the background object
    // as specified by --window--
    PickerShape {
        id: selectorMask
        pickerColor: parent.pickerColor
        anchors.fill: parent
        opacity: 0.55
        z: 1
    }

    // this is an opaque backsplash, placed between the selectormask
    // and the background
    PickerShape {
        id: selector
        anchors.fill: parent
        edgeRadius: 10
        opacity: 1.0
        z: -1
    }

    PickerInterior {
        z: 2
        anchors {
            fill: parent
            centerIn: parent
        }
    }
}
