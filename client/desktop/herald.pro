TEMPLATE = app

QT      += quick svg xml widgets
VERSION = 0.0.1
CONFIG  += c++11

# The following define makes your compiler emit warnings if you use
# any Qt feature that has been marked deprecated (the exact warnings
# depend on your compiler). Refer to the documentation for the
# deprecated API to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# silence compiler warnings from Qt headers
QMAKE_CXXFLAGS += -isystem $$[QT_INSTALL_HEADERS]
# silence the project warning about unsuported SDK
CONFIG += sdk_no_version_check

RESOURCES += \
    qml.qrc

SOURCES   += main.cpp

# libherald headers, source, and libs
include(../foundation/foundation.pri)

macx {
    ICON = ../foundation/icons/herald.icns
}


CONFIG(debug, debug|profile|release) {
    linux {
        CONFIG+=sanitizer
        CONFIG+=sanitize_address sanitize_memory sanitize_undefined
    }
}

unix:!macx {
    isEmpty(PREFIX) {
        PREFIX = /usr
    }

    target.path = $$PREFIX/bin

    shortcutfiles.files = herald.desktop
    shortcutfiles.path = $$PREFIX/share/applications/
    data.files += ../foundation/icons/herald.png
    data.path = $$PREFIX/share/pixmaps/

    INSTALLS += shortcutfiles
    INSTALLS += data

    INSTALLS += target

    DISTFILES += \
        herald.desktop \
        ../foundation/icons/herald.png
}
