QT += quick svg xml
VERSION = 0.0.1

INCLUDEPATH += $$PWD/../../libherald/qt_ffi \
               $$PWD

QML_IMPORT_PATH +=\
    $$PWD

HEADERS += \
     $$PWD/../../libherald/qt_ffi/Bindings.h \
     $$PWD/androidhelper.h


SOURCES += \
    $$PWD/../../libherald/qt_ffi/Bindings.cpp \
    $$PWD/androidhelper.cpp


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

#platform specific settings
#iphonesimulator {
#    LIBS += $${PWD}/../../target/x86_64-apple-ios/$${RUST_BUILD_TYPE}/libherald.a \
#        -l sqlite3
#    ENABLE_BITCODE = NO
#    HEADERS += $$PWD/objectiveutils.h
#    SOURCES += $$PWD/objectiveutils.mm
#}

iphoneos {
   Q_ENABLE_BITCODE.name = ENABLE_BITCODE
   Q_ENABLE_BITCODE.value = NO
   QMAKE_MAC_XCODE_SETTINGS += Q_ENABLE_BITCODE

    LIBS += $${PWD}/../../target/aarch64-apple-ios/$${RUST_BUILD_TYPE}/libherald.a \
        -l sqlite3
    HEADERS += $$PWD/objectiveutils.h
    SOURCES += $$PWD/objectiveutils.mm
}


macx {
  LIBS += -L $${PWD}/../../target/$${RUST_BUILD_TYPE} -lherald
  HEADERS += $$PWD/objectiveutils.h
  SOURCES += $$PWD/objectiveutils.mm
}


android {
      # QMAKE_LFLAGS += -nostdlib++
     QT += androidextras
     ANDROID_ABIS = armeabi-v7a
     ANDROID_NDK_PLATFORM = android-28
     ANDROID_API_VERSION = 28
     LIBS +=  $${PWD}/../../target/armv7-linux-androideabi/$${RUST_BUILD_TYPE}/libherald.a
     HEADERS +=
     SOURCES +=
  }


RESOURCES += \
    $$PWD/icons/icons.qrc \
    $$PWD/qml/commonQml.qrc

