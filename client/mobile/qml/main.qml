import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Window 2.2
import "./ChatView" as ChatView
import "./State" as State
import "./LoginPage" as LoginPage

ApplicationWindow {
    visible: true
    width: 300
    height: 500

    StackView {
        id: mainView
        anchors.fill: parent
    }

    Rectangle {
        anchors.fill: parent
        color: "orange"
    }

    State.AppState {
        mainView: mainView
    }
}
