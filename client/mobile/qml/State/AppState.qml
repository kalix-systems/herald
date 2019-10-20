import QtQuick 2.12
import QtQuick.Controls 2.12
import "../ConversationView" as CVView
import "../LoginPage" as LoginPage

Item {
    property StackView stackView
    property Component lpMain: LoginPage.LoginLandingPage {}
    property Component cvMain: CVView.ConversationViewMain {}

    states: [
        State {
            when: !heraldState.configInit
            name: "setup"
        },
        State {
            when: heraldState.configInit
            name: "contact"
            StateChangeScript {
                script: stackView.replace(cvMain)
            }
        },
        State {
            name: "config"
            PropertyChanges {}
        }
    ]
}
