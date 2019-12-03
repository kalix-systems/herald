import QtQuick 2.13
import QtQuick.Controls 2.12

Popup {
    property string sourcePath
    Image {
        source: sourcePath
    }
}
