import LibHerald 1.0
import QtGraphicalEffects 1.13
import QtQuick 2.13
import QtQuick.Controls 2.12
import "./Controls"

Page {
    id: loginLandingPage

    property color bgEndColor: "#5c7598"
    property color bgStartColor: "#5c7598"

    anchors.fill: parent

    background: Rectangle {
        color: bgStartColor
    }

    LoginField {
        id: entryField
        anchors {
            horizontalCenter: newAccButton.horizontalCenter
            bottom: serverAddrTextField.top
            bottomMargin: CmnCfg.units.dp(15)
        }
    }

    TextField {
        id: serverAddrTextField
        anchors {
            horizontalCenter: newAccButton.horizontalCenter
            bottom: serverPortTextField.top
            bottomMargin: CmnCfg.units.dp(15)
        }
        width: parent.width - 2 * CmnCfg.megaMargin
        placeholderText: qsTr("Server address")
        text: "54.213.103.80"
    }

    TextField {
        id: serverPortTextField
        anchors {
            horizontalCenter: newAccButton.horizontalCenter
            bottom: newAccButton.top
            bottomMargin: CmnCfg.units.dp(30)
        }
        width: parent.width - 2 * CmnCfg.megaMargin
        placeholderText: qsTr("Server port")
        text: "8080"
    }

    LoginButton {
        id: newAccButton

        lbText: qsTr("Register New Device")
        lbColor: bgStartColor

        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: connectAccountButton.top
            bottomMargin: CmnCfg.units.dp(15)
        }

        onClicked: {
            Herald.registerNewUser(entryField.text.trim(),
                                   serverAddrTextField.text.trim(),
                                   serverPortTextField.text.trim())
        }
    }

    LoginButton {
        id: connectAccountButton

        lbText: qsTr("Connect Device to Existing Account")
        lbColor: bgStartColor

        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: registrationFailureMessage.top
            bottomMargin: CmnCfg.units.dp(15)
        }

        onClicked: {
            loginPageLoader.sourceComponent = connectDevicePage
        }
    }

    Text {
        id: registrationFailureMessage
        // TODO mostly just a place holder
        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: parent.bottom
            bottomMargin: loginLandingPage.height / 3
        }
        text: ""
        color: "red"
        visible: false

        Connections {
            target: Herald
            onRegistrationFailureCodeChanged: {
                const code = Herald.registrationFailureCode
                if (code !== undefined) {
                    switch (code) {
                    case 0:
                        registrationFailureMessage.text = qsTr("User id taken")
                        break
                    case 1:
                        registrationFailureMessage.text = qsTr("Key taken")
                        break
                    case 2:
                        registrationFailureMessage.text = qsTr("Bad signature")
                        break
                    case 3:
                        registrationFailureMessage.text = qsTr(
                                    "Registration failed")
                        break
                    default:
                        registrationFailureMessage.text = qsTr(
                                    "Registration failed")
                    }

                    registrationFailureMessage.visible = true
                }
            }
        }
    }
}
