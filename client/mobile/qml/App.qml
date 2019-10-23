import QtQuick 2.0
import QtQuick.Controls 2.12
import "./State" as State

Component {
    Item {
    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: appState.cvMain
    }

    State.AppState {
        id: appState
        stackView: mainView
    }
    }
}
