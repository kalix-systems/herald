import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0

import "../State/js/transitions.js" as Transitions

Page {
    id: contactViewMain

    header: ToolBar {
        background: Rectangle {
            color: QmlCfg.palette.secondaryColor
        }
        RowLayout {
            CVToolButton {
                imageSource: ""
            }
            CVToolButton {
                imageSource: ""
            }
        }
    }

    Loader {
        id: configLoader
        active: heraldState.configInit
        Config {
            id: config
        }
    }
}
