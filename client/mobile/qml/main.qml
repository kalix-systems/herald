import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./ConversationView" as CVView
import "./ChatView" as ChatView
import "./Errors" as Errors
import "./LoginPage" as LoginPage
import "./State" as State

ApplicationWindow {
    visible: true
    width: 300
    height: 500

    // contains back end state. Login status,
    // and boolean configuration init status
    property alias heraldState: heraldGlobals.heraldState
    // handles all network polling, emit tryPollUpdate upon
    // receiving and update
    property alias networkHandle: heraldGlobals.networkHandle
    // utility code, meant to reduce the amount of js laying
    // around the code base
    property alias heraldUtils: heraldGlobals.heraldUtils

    Loader {
        id: configLoader
        active: heraldState.configInit
        sourceComponent: Config {
            id: config
        }
    }

    // displays error dialog upon output from
    // libherald, meant as a debugging tool
    Errors.ErrorHandler {}

    // initializer for LibHerald models
    State.HeraldGlobals {
        id: heraldGlobals
    }

    // handles transitions for the main stack view, initializes all
    // views, and sets properties to the correct values.
    State.AppState {
        id: appState
        stackView: mainView
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: appState.cvMain
    }
}
