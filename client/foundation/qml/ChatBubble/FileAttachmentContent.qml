import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

Column {
    id: wrapperCol

    property real maxWidth: Math.min(contentRoot.maxWidth, 600)
    property var docParsed

    spacing: 0

    Component.onCompleted: JSON.parse(documentAttachments).forEach(
                               function (doc) {
                                   docModel.append(doc)
                               })

    ListModel {
        id: docModel
    }

    DocFileItem {
        model: docModel
    }
}
