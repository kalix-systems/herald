import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import "./utils.mjs" as Utils

/// --- displays a list of contacts
// BNOTE: this looks like margin fudging
Image {
    width: rowHeight - 10
    height: rowHeight - 10
    source: Utils.safeToQrcURI(pfpUrl)
}
