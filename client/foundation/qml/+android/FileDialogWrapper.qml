import QtQuick 2.14
import LibHerald 1.0

Item {
    property MobileHelper mobHelp
    property string folder
    property var nameFilters
    property string currentFile
    property var currentFiles
    signal selectionAccepted
}
