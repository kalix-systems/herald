import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../common" as Common
import QtQml 2.13
import QtQuick.Controls 2.5
import Qt.labs.platform 1.0

Menu {
    property int chosenPeriod: conversationItem.expirationPeriod
    property string chosenTimer: timerModel.get(chosenPeriod).path
    ListModel {
        id: timerModel

        ListElement {
            name: "Off"
            path: "qrc:/timer-icons/off.svg"
        }

        ListElement {
            name: "1 minute"
            path: "qrc:/timer-icons/1min.svg"
        }
        ListElement {
            name: "1 hour"
            path: "qrc:/timer-icons/1h.svg"
        }
        ListElement {
            name: "1 day"
            path: "qrc:/timer-icons/1d.svg"
        }
        ListElement {
            name: "1 week"
            path: "qrc:/timer-icons/1w.svg"
        }
        ListElement {
            name: "1 month"
            path: "qrc:/timer-icons/1mo.svg"
        }
        ListElement {
            name: "1 year"
            path: "qrc:/timer-icons/1y.svg"
        }
    }

    id: optMenu
    Instantiator {
        model: timerModel

        MenuItem {
            text: name
            onTriggered: {
                conversationItem.expirationPeriod = index
            }
        }

        onObjectAdded: optMenu.insertItem(index, object)
    }
}
