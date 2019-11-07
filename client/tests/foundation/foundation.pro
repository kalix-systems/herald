CONFIG += warn_on qmltestcase

TEMPLATE = app

include(../../foundation/foundation.pri)

DISTFILES += \
    tst_controls.qml

RESOURCES += ../../desktop/qml.qrc

SOURCES += \
    main.cpp
