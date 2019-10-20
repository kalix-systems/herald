import LibHerald 1.0

Errors {
    id: self
    property var errPopup: ErrorPopup {}
    onTryPollChanged: {
        const errMsg = nextError()
        if (errMsg !== "") {
            errPopup.errorMsg = errMsg
            errPopup.open()
        }
    }
}
