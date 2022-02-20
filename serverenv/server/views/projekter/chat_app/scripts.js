function logged_in() {
    let user_id = localStorage.getItem("usr_id")
    let token = window.localStorage.getItem("usr_token");


    if (user_id == null || token == null) {
        return false;
    }

    let success = false;
    $.ajax({
        url: "/projekter/chat_app/verify/" + user_id + "/" + token,
        async: false,
        success: data => {
            success = data.success;
        }
    });
    return success;
}

function log_out() {
    window.localStorage.clear();
    window.location.pathname = "/projekter/chat_app/login";
}


function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}