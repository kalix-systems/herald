import LibHerald 1.0
import QtGraphicalEffects 1.13
import QtQuick 2.13
import QtQuick.Controls 2.12

Page {
    id: loginLandingPage

    anchors {
        fill: parent
    }

    //property color bgEndColor: "#5c7598"
    property color bgStartColor: "#5c7598"

    background: Rectangle {
        color: bgStartColor
    }

    LoginField {
        anchors {
            horizontalCenter: newAccButton.horizontalCenter
            bottom: newAccButton.top
            bottomMargin: QmlCfg.units.gu(3)
        }
    }

    LoginButton {
        id: newAccButton

        lbText: "Register New Device"
        lbColor: bgStartColor

        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: parent.bottom
            bottomMargin: QmlCfg.units.gu(12)
        }
    }
}
