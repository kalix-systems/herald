function func(target) {
        while(target.Component.status === Component.Loading ) {}
        WorkerScript.sendMessage({target.ContentHeight});
}
