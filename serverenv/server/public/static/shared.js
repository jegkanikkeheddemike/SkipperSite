//if user is not logged in, then redirect to login
if (!logged_in()) {
    if (window.location.pathname != "/login")
        window.location.pathname = "/login"
} else {
    if (window.location.pathname != "/chat")
        window.location.pathname = "/chat"
}

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