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
$("#login_form").ajaxForm(function (response) {
    login(response);
});
//Bind ajax_forms to the form
//MUST be loaded after the DOM
$("#signup_form").ajaxForm(function (response) {
    signup(response);
});

function signup(res) {
    console.log(res);
    if (typeof (res) == typeof ("")) {
        app.failed_signup = true;
        app.signup_error = res.substring(7);
    } else {
        login(res);
    }
}


function login(res) {
    console.log(res);
    if (typeof (res) == typeof ("")) {
        app.failed_login = true;
    } else {

        let user = res;

        console.log(user);

        window.localStorage.setItem("usr_token", user.private.token);
        window.localStorage.setItem("usr_name", user.public.username);
        window.localStorage.setItem("usr_id", user.public.id);
        window.location.pathname = "/chat";
    }
}