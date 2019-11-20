import LibHerald 1.0

Errors {
    id: self
    property var errPopup: ErrorPopup {
    }
    onTryPollChanged: {
        var errMsg = nextError()
        if (errMsg !== "") {
            errPopup.errorMsg = errMsg
            errPopup.open()
        }
    }
}
