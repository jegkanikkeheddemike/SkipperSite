function logged_in(){
    let token = window.localStorage.getItem("usr_token");
    if (token) {
        return true;
    }
    return false;
}

function log_out() {
    window.localStorage.clear();
    window.location.pathname = "/login";
}


function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}