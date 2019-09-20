QT += testlib
QT -= gui

debug {
 RUST_BUILD_TYPE = debug
}
else {
  release | profile {
   RUST_BUILD_TYPE = release
  }
}

INCLUDEPATH += ../../../libherald/qt_ffi
DEPENDPATH += ./../../libherald/qt_ffi

CONFIG += qt console warn_on depend_includepath testcase
CONFIG -= app_bundle

TEMPLATE = app

SOURCES +=  tst_libherald.cpp \
../../../libherald/qt_ffi/Bindings.cpp

HEADERS +=  ../../../libherald/qt_ffi/Bindings.h

# I think this is okay, as building for OS X implies the host is the same as
# the target
macx {
  LIBS += -L $${PWD}/../../../libherald/target/$${RUST_BUILD_TYPE} -lherald
}

linux {
  LIBS += $${PWD}/../../../libherald/target/$${RUST_BUILD_TYPE}/libherald.so
}
