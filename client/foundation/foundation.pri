QT += quick svg xml
VERSION = 0.0.1

INCLUDEPATH += $$PWD/../../libherald/qt_ffi \
               $$PWD

QML_IMPORT_PATH +=\
    $$PWD


HEADERS += \
     $$PWD/../../libherald/qt_ffi/Bindings.h \


SOURCES += \
    $$PWD/../../libherald/qt_ffi/Bindings.cpp \


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
CONFIG(debug, iphonesimulator & !iphoneos) {
    iphonesimulator {
        LIBS += $${PWD}/../../target/x86_64-apple-ios/$${RUST_BUILD_TYPE}/libherald.a \
            -l sqlite3
        ENABLE_BITCODE = NO
        HEADERS += $$PWD/objectiveutils.h
        SOURCES += $$PWD/objectiveutils.mm
    }
}

CONFIG(debug|release, iphoneos & !iphonesimulator) {
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
     QT += androidextras
     ANDROID_ABIS = armeabi-v7a

     LIBS +=  $${PWD}/../../target/armv7-linux-androideabi/$${RUST_BUILD_TYPE}/libherald.a
     HEADERS +=  $$PWD/androidhelper.h
     SOURCES +=  $$PWD/androidhelper.cpp

     ANDROID_PACKAGE_SOURCE_DIR = $$PWD/android-sources

DISTFILES += \
      $$PWD/android-sources/AndroidManifest.xml \
      $$PWD/android-sources/build.gradle \
      $$PWD/android-sources/gradle/wrapper/gradle-wrapper.jar \
      $$PWD/android-sources/gradle/wrapper/gradle-wrapper.properties \
      $$PWD/android-sources/gradlew \
      $$PWD/android-sources/gradlew.bat \
      $$PWD/android-sources/res/values/libs.xml \
      $$PWD/android-sources/src/NotificationBuilder.java

}


RESOURCES += \
    $$PWD/icons/icons.qrc \
    $$PWD/qml/commonQml.qrc

