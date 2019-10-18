import QtQuick 2.12
import QtQuick.Controls 2.12
import "../ContactsView" as ContactView
import "../LoginPage" as LoginPage

Item {
    property Component view
    property StackView stackView
    property Component cvMain: ContactView.ContactViewMain {}
    property Component lpMain: LoginPage.LoginLandingPage {}

    state: "setup"

    states: [
        State {
            name: "setup"
            PropertyChanges {}
        },
        State {
            name: "contact"
            PropertyChanges {}
        },
        State {
            name: "config"
            PropertyChanges {}
        }
    ]
}
