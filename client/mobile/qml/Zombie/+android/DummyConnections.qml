import QtQuick 2.14

Item {
    Connections {
        target: root
        onClosing: {
            if (mainView.depth > 1) {
                mainView.pop()
                close.accepted = false
            }
        }
    }
}
