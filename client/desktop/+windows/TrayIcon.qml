import QtQuick 2.14
import QtQuick.Controls 2.14
import Qt.labs.platform 1.1
import LibHerald 1.0

SystemTrayIcon {
    id: tray
    icon.source: "qrc:/herald.png"
    Component.onCompleted: show()
    Connections {
        target: Herald.notifications
        onNotify: {
            if (!root.active) {
                const notif = JSON.parse(Herald.notifications.nextNotif())
                tray.showMessage("Herald", notif.msg)
            }
        }
    }
}
