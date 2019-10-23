import LibHerald 1.0
import QtGraphicalEffects 1.13
import QtQuick 2.13
import QtQuick.Controls 2.12
import "./Controls"

Page {
    id: loginLandingPage

    property color bgEndColor: "#5c7598"
    property color bgStartColor: "#5c7598"

    background: Rectangle {
        color: bgStartColor
    }

    LoginField {
        id: entryField
        anchors {
            horizontalCenter: newAccButton.horizontalCenter
            bottom: newAccButton.top
            bottomMargin: CmnCfg.units.dp(30)
        }
    }

    LoginButton {
        id: newAccButton

        lbText: "Register New Device"
        lbColor: bgStartColor

        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: parent.bottom
            bottomMargin: loginLandingPage.height / 3
        }

        onClicked: registerUser()
    }

    function registerUser() {
        if (networkHandle.registerNewUser(entryField.text.trim())) {
            heraldState.configInit = true
        }
    }
}
