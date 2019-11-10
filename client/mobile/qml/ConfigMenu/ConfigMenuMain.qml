import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./Controls"
import "../Common" as Common

Page {
    id: configPage
    header: ConfigHeader {}
    ScrollView {
        anchors.fill: parent
        Column {
            id: configContent
            Common.HeaderText {
                text: "Status"
            }
            Common.Divider {}
            Common.HeaderText {
                text: "Account"
            }
            Common.Divider {}
            Common.HeaderText {
                text: "Appearance"
            }
            Common.Divider {}
            Common.HeaderText {
                text: "Notifications"
            }
            Common.Divider {}
            Common.HeaderText {
                text: "Advanced"
            }
        }
    }
}
