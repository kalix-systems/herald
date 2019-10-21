import QtQuick 2.13

QtObject {
    property list<QtObject> themes: [
        Light {},
        SolarizedDark {},
        Dark {},
        EarthTones {},
        SolarizedLight {}
    ]
    // Font to be used for all ui elements except inside chat bubbles
    // EuroStile is licensed
    property FontLoader labelFont: FontLoader {}
    // explicit, chatbubble only font
    property FontLoader chatFont: FontLoader {
        source: "../Assets/IBMPlexSans-Regular.ttf"
    }
    // font for code blocks, should be user configurable
    property FontLoader monoSpaceFont: FontLoader {
        source: "../Assets/Monoid-Regular.ttf"
    }
}
