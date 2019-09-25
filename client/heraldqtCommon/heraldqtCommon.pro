TEMPLATE = subdirs

SUBDIRS += \
    libCommonQml \
    libHerald \
    libHeraldTests

libHeraldTests.subdir  = ./libHeraldTests
libHerald.subdir       = ./libHerald
libCommonQml.subdir    = ./libCommonQml
