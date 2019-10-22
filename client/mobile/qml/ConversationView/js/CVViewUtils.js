function initialize(name) {
    const tokens = name.split(' ').slice(0, 3)
    var str = ""
    tokens.forEach(function anon(string) {
        str += string[0].toUpperCase()
    })
    return str
}

function friendlyTimestamp(epochtimestamp_ms) {}

function searchBarTr() {
    appState.state = "search"
}
