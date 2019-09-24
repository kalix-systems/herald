# libherald headers, source, and libs
include( ../libHerald/libHerald.pri )

QT += testlib
QT += quick svg xml
QT -= gui
VERSION = 0.0.1


CONFIG += qt console warn_on depend_includepath testcase
CONFIG -= app_bundle

TEMPLATE = app

SOURCES +=  tst_libherald.cpp
