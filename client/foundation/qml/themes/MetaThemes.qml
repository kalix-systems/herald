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
    property FontLoader cairo: FontLoader {
        source: "../Assets/Cairo-Regular.ttf"
    }

    property FontLoader cairoDemiBold: FontLoader {
        source: "../Assets/Cairo-DemiBold.ttf"
    }

    property FontLoader cairoBold: FontLoader {
        source: "../Assets/Cairo-Bold.ttf"
    }

    // TODO not just a chatbubble font, it's the default for the app UI,
    // rename accordingly
    // explicit, chatbubble only font
    property FontLoader chatFont: FontLoader {
        source: "../Assets/IBMPlexSans-Regular.ttf"
    }

    property FontLoader chatFontMedium: FontLoader {
        source: "../Assets/IBMPlexSans-Medium.ttf"
    }

    property FontLoader chatFontBold: FontLoader {
        source: "../Assets/IBMPlexSans-Bold.ttf"
    }

    property FontLoader chatFontDemiBold: FontLoader {
        source: "../Assets/IBMPlexSans-DemiBold.ttf"
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
