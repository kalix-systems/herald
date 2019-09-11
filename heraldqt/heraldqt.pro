QT += quick
QT += widgets

CONFIG += c++11

# The following define makes your compiler emit warnings if you use
# any Qt feature that has been marked deprecated (the exact warnings
# depend on your compiler). Refer to the documentation for the
# deprecated API to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

INCLUDEPATH += $$PWD/../libherald/qt_ffi

SOURCES += \
        main.cpp \
        $${PWD}/../libherald/qt_ffi/Bindings.cpp

HEADERS += $${PWD}/../libherald/qt_ffi/Bindings.h

# silence compiler warnings from Qt headers
QMAKE_CXXFLAGS += -isystem $$[QT_INSTALL_HEADERS]

# set build type for Rust library
debug {
 RUST_BUILD_TYPE = debug
}
else {
  release | profile {
   RUST_BUILD_TYPE = release
  }
}

# Only for simulator right now
android {

    $$QT_FILE_SELECTORS += mobile
    QMAKE_LFLAGS += -nostdlib++
    LIBS +=  $${PWD}/../libherald/target/i686-linux-android/$${RUST_BUILD_TYPE}/libherald.a
}

# Only for simulator right now
iphonesimulator {
    # important IOS build Notes
    # IOS will not work because of code signing, but it would with this...?
    # IOS simulator and IOS builds must be completed from XCode. There is a bug in qmake.
    #$$QT_FILE_SELECTORS += mobile
    LIBS +=  $${PWD}/../libherald/target/x86_64-apple-ios/$${RUST_BUILD_TYPE}/libherald.a \
        -lsqlite3
}

# I think this is okay, as building for OS X implies the host is the same as
# the target
macx {
  LIBS += -L $${PWD}/../libherald/target/$${RUST_BUILD_TYPE} -lherald
}

linux {
  LIBS += $${PWD}/../libherald/target/$${RUST_BUILD_TYPE}/libherald.so
}

RESOURCES += qml.qrc

# Additional import path used to resolve QML modules in Qt Creator's code model
QML_IMPORT_PATH =

# Additional import path used to resolve QML modules just for Qt Quick Designer
QML_DESIGNER_IMPORT_PATH =

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

DISTFILES +=
