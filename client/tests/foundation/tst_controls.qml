import QtQuick 2.0
import QtTest 1.0
import LibHerald 1.0

TestCase {
    name: "controls"

    function test_redline_const() {
        compare(CmnCfg.margin, 12, "Margins are redlined")
    }
}
