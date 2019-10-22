TEMPLATE = app

QT      += quick svg xml
VERSION = 0.0.1
CONFIG  += c++11

# The following define makes your compiler emit warnings if you use
# any Qt feature that has been marked deprecated (the exact warnings
# depend on your compiler). Refer to the documentation for the
# deprecated API to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# libherald headers, source, and libs
include(../foundation/foundation.pri)

# silence compiler warnings from Qt headers
QMAKE_CXXFLAGS += -isystem $$[QT_INSTALL_HEADERS]
# silence the project warning about unsuported SDK
CONFIG+=sdk_no_version_check

RESOURCES +=  qml.qrc
SOURCES   += main.cpp

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target


