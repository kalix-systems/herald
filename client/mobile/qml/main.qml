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

    property alias heraldState: heraldGlobals.heraldState
    property alias networkHandle: heraldGlobals.networkHandle
    property alias heraldUtils: heraldGlobals.heraldUtils

    State.HeraldGlobals {
        id: heraldGlobals
    }

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
