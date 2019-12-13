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

    linux:!android {
        LIBS += $${PWD}/../../target/debug/libherald.so
    }
}

CONFIG(profile, debug|profile|release) {
    RUST_BUILD_TYPE = release

    linux:!android {
      LIBS += $${PWD}/../../target/$${RUST_BUILD_TYPE}/libherald.a
      LIBS += -ldl
      LIBS += -ldbus-1
    }
}
CONFIG(release, debug|profile|release) {
    RUST_BUILD_TYPE = release

    linux:!android {
      LIBS += $${PWD}/../../target/$${RUST_BUILD_TYPE}/libherald.a
      LIBS += -ldl
      LIBS += -ldbus-1
    }
}

# platform specific settings
iphonesimulator {
    LIBS += $${PWD}/../../target/x86_64-apple-ios/$${RUST_BUILD_TYPE}/libherald.a \
        -l sqlite3
}

macx {
  LIBS += -L $${PWD}/../../target/$${RUST_BUILD_TYPE} -lherald
}


android {
      # QMAKE_LFLAGS += -nostdlib++
     ANDROID_ABIS= armeabi-v7a
     ANDROID_NDK_PLATFORM = android-28
     ANDROID_API_VERSION = 28
     LIBS +=  $${PWD}/../../target/armv7-linux-androideabi/$${RUST_BUILD_TYPE}/libherald.a
  }


RESOURCES += \
    $$PWD/icons/icons.qrc \
    $$PWD/qml/commonQml.qrc

