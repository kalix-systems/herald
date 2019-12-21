import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtGraphicalEffects 1.12
import LibHerald 1.0

// wrapper component for the file list component.
// TODO: move into the DocFileItem, this wrapping is inane
Column {
    id: wrapperCol

    property real maxWidth: Math.min(contentRoot.maxWidth, 600)
    property var docParsed: JSON.parse(documentAttachments)

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
