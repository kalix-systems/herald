import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0

TextField {
    id: lpTextField

    placeholderText: "Enter Username..."

    width: QmlCfg.units.gu(15)

    background: Rectangle {
        color: "#FFFFFF"
        radius: QmlCfg.radius
    }
}
