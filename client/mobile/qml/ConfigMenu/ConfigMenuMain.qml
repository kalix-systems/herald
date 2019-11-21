import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "./Controls"
import "../Common" as Common

Page {
    id: configPage
    header: ConfigHeader {
    }

    ColumnLayout {
        id: configContent
        width: parent.width
        ConfigSection {
            title: "Account"
            content: Column {

                Row {
                    Common.HeaderText {
                        font.pointSize: 18
                        text: "Username : "
                        leftPadding: CmnCfg.margin
                    }
                    Common.HeaderText {
                        font.pointSize: 18
                        text: configModel.name
                        color: CmnCfg.palette.secondaryTextColor
                    }
                }

                Common.HeaderText {
                    font.pointSize: 12
                    text: "     The primary identifying contact information of your account,\
any of your contacts must know this identifier exactly. It is not searchable, nor can it be changed."
                    color: CmnCfg.palette.secondaryTextColor
                    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                    leftPadding: CmnCfg.margin
                    width: parent.width
                }
            }
        }

        ConfigSection {
            title: "Appearance"
        }

        ConfigSection {
            title: "Notifications"
        }

        ConfigSection {
            title: "Advanced"
        }

        ConfigSection {
            title: "About"
        }
    }
}
