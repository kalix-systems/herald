
QT += quick svg xml
VERSION = 0.0.1

INCLUDEPATH += $$PWD/../../libherald/qt_ffi

HEADERS += \
     $$PWD/../../libherald/qt_ffi/Bindings.h

SOURCES += \
    $$PWD/../../libherald/qt_ffi/Bindings.cpp


# set build type for Rust library
CONFIG(debug, debug|profile|release) {
    RUST_BUILD_TYPE = debug
}
CONFIG(profile, debug|profile|release) {
    RUST_BUILD_TYPE = release
}
CONFIG(release, debug|profile|release) {
    RUST_BUILD_TYPE = release
}





iphonesimulator {
    LIBS +=  $${PWD}/../../target/x86_64-apple-ios/$${RUST_BUILD_TYPE}/libherald.a \
        -lsqlite3
}

macx {
  LIBS += -L $${PWD}/../../target/$${RUST_BUILD_TYPE} -lherald
}

linux {
android {
    # QMAKE_LFLAGS += -nostdlib++
   ANDROID_NDK_PLATFORM = android-28
   ANDROID_API_VERSION = 28
   #LIBS +=  $${PWD}/../../target/x86_64-linux-android/$${RUST_BUILD_TYPE}/libherald.a
   LIBS +=  $${PWD}/../../target/armv7-linux-androideabi/$${RUST_BUILD_TYPE}/libherald.a
} else {
    LIBS += $${PWD}/../../target/$${RUST_BUILD_TYPE}/libherald.so
}
}

RESOURCES += \
    $$PWD/icons/icons.qrc \
    $$PWD/qml/commonQml.qrc

