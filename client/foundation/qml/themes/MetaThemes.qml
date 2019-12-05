import QtQuick 2.13

QtObject {
    property list<QtObject> themes: [
        Light {},
        SolarizedDark {},
        Dark {},
        EarthTones {},
        SolarizedLight {}
    ]

    // Title Fonts
    property FontLoader labelFont: FontLoader {
        source: "../Assets/Jura-Regular.ttf"
    }
    property FontLoader labelFontBold: FontLoader {
        source: "../Assets/Jura-Bold.ttf"
    }

    // explicit, chatbubble only font
    property FontLoader chatFont: FontLoader {
        source: "../Assets/IBMPlexSans-Regular.ttf"
    }
    property FontLoader chatFontBold: FontLoader {
        source: "../Assets/IBMPlexSans-Bold.ttf"
    }
    property FontLoader chatFontSemiBold: FontLoader {
        source: "../Assets/IBMPlexSans-SemiBold.ttf"
    }
    property FontLoader chatFontItalic: FontLoader {
        source: "../Assets/IBMPlexSans-Italic.ttf"
    }
    property FontLoader chatFontItalicBold: FontLoader {
        source: "../Assets/IBMPlexSans-BoldItalic.ttf"
    }

    // font for code blocks, should be user configurable
    property FontLoader monoSpaceFont: FontLoader {
        source: "../Assets/Monoid-Regular.ttf"
    }
}
