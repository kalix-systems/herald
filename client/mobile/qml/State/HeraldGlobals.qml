import LibHerald 1.0
import QtQuick 2.13
import QtQuick.Controls 2.12

Item {
    property alias conversationsModel: conversationsModel
    property alias usersModel: usersModel
    property alias configModel: configModel

    Conversations {
        id: conversationsModel
    }

    Users {
        id: usersModel
    }

    Config {
        id: configModel
    }
}
