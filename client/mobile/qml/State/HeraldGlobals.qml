import LibHerald 1.0
import QtQuick 2.13
import QtQuick.Controls 2.12

Item {

    property alias heraldUtils: heraldUtils
    property alias heraldState: heraldState
    property alias networkHandle: networkHandle

    HeraldUtils {
        id: heraldUtils
    }

    HeraldState {
        id: heraldState
    }

    NetworkHandle {
        id: networkHandle
    }
}
