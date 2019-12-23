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
            bottomMargin: CmnCfg.units.dp(30)
        }
    }

    TextField {
        id: serverAddrTextField
        anchors {
            horizontalCenter: newAccButton.horizontalCenter
            bottom: serverPortTextField.top
            bottomMargin: CmnCfg.units.dp(30)
        }
        width: CmnCfg.units.gu(15)
        placeholderText: qsTr("Server address")
    }

    TextField {
        id: serverPortTextField
        anchors {
            horizontalCenter: newAccButton.horizontalCenter
            bottom: newAccButton.top
            bottomMargin: CmnCfg.units.dp(30)
        }
        width: CmnCfg.units.gu(15)
        placeholderText: qsTr("Server port")
    }

    LoginButton {
        id: newAccButton

        lbText: qsTr("Register New Device")
        lbColor: bgStartColor

        anchors {
            horizontalCenter: parent.horizontalCenter
            bottom: registrationFailureMessage.top
            bottomMargin: CmnCfg.units.dp(30)
        }

        onClicked: {
            Herald.registerNewUser(entryField.text.trim(),
                                   serverAddrTextField.text.trim(),
                                   serverPortTextField.text.trim())
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
