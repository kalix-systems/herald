import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQml 2.13
import QtQuick.Controls 2.5
import Qt.labs.platform 1.0

//version of timer options for the builder
Menu {
    id: optMenu

    // Item in `Conversations` model
    property var conversationItem
    property MessageBuilder builder
    property int chosenPeriod: builderDefault ? 0 : (builder.expirationPeriod + 1)

    property string chosenTimer: timerModel[chosenPeriod].path
    property bool builderDefault: true
    font: CmnCfg.chatFont.name

    // TODO real icon
    property var timerModel: [{
            "name": qsTr("Conversation default"),
            "path": "qrc:/timer-option-icons/blank-dark.svg"
        }, {
            "name": qsTr("Off"),
            "path": "qrc:/timer-option-icons/off-dark.svg"
        }, {
            "name": qsTr("30 seconds"),
            "path": "qrc:/timer-option-icons/30s-dark.svg"
        }, {
            "name": qsTr("1 minute"),
            "path": "qrc:/timer-option-icons/1min-dark.svg"
        }, {
            "name": qsTr("30 minutes"),
            "path": "qrc:/timer-option-icons/30min-dark.svg"
        }, {
            "name": qsTr("1 hour"),
            "path": "qrc:/timer-option-icons/1h-dark.svg"
        }, {
            "name": qsTr("12 hours"),
            "path": "qrc:/timer-option-icons/12h-dark.svg"
        }, {
            "name": qsTr("1 day"),
            "path": "qrc:/timer-option-icons/1d-dark.svg"
        }, {
            "name": qsTr("1 week"),
            "path": "qrc:/timer-option-icons/1w-dark.svg"
        }, {
            "name": qsTr("1 month"),
            "path": "qrc:/timer-option-icons/1mo-dark.svg"
        }, {
            "name": qsTr("1 year"),
            "path": "qrc:/timer-option-icons/1y-dark.svg"
        }]

    Instantiator {
        model: timerModel

        MenuItem {
            text: timerModel[index].name
            checkable: true
            checked: {
                builderDefault ? (index === (0)) : builder.expirationPeriod === index - 1
            }
            onTriggered: {

                if (index === 0) {
                    builderDefault = true
                    return builder.setExpirationPeriod(
                                conversationItem.expirationPeriod)
                }

                builder.setExpirationPeriod(index - 1)
                builderDefault = false
            }
        }

        onObjectAdded: {
            optMenu.insertItem(index, object)
        }
    }
}
