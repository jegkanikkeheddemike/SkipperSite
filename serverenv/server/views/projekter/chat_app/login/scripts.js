if (logged_in()) {
    location.pathname = "/projekter/chat_app/chat/"
}

var app = new Vue({
    el: '#app',
    data: {
        failed_login: false,
        failed_signup: false,
        signup_error: ""
    }
});

//Bind ajax_forms to the form
//MUST be loaded after the DOM
$("#login_form").ajaxForm(response => {
    login(response);
});
//Bind ajax_forms to the form
//MUST be loaded after the DOM
$("#signup_form").ajaxForm(function (response) {
    signup(response);
});

function signup(res) {
    if (res.success) {
        login(res);
    } else {
        app.signup_error = res.data;
        app.failed_signup = true;
    }
}


function login(res) {
    if (res.success) {

        let user = res.data;
        window.localStorage.setItem("usr_token", user.private.token);
        window.localStorage.setItem("usr_name", user.public.username);
        window.localStorage.setItem("usr_id", user.public.id);
        window.location.pathname = "/projekter/chat_app/chat";
    } else {
        app.failed_login = true;
    }
}