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

    State.HeraldGlobals {
        id: heraldGlobals
    }

    State.AppState {
        id: appstate
        view: heraldGlobals.heraldState.configInit ? appstate.cvMain : appstate.lpMain
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: appstate.view
    }
}
