import QtQuick 2.13
import QtQuick.Controls 2.12
import "./ContactsView" as Contactview
import "./ChatView" as ChatView
import "./State" as State
import "./LoginPage" as LoginPage

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

    // initializer for LibHerald models
    State.HeraldGlobals {
        id: heraldGlobals
    }

    // handles transitions for the main stack view, initializes all
    // views, and sets properties to the correct values.
    State.AppState {
        id: appstate
        view: heraldState.configInit ? appstate.cvMain : appstate.lpMain
        stackView: mainView
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: appstate.view
    }
}
