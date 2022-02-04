var app = new Vue({
    el: '#app',
    data: {
        failed_login: false,
        failed_signup: false
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
    if (res.startsWith("ERR: ")) {

    } else {
        login(res);
    }
}


function login(res) {
    console.log(res);
    if (res == "LOGIN_ERR") {
        app.failed_login = true;
    } else {

        let user = JSON.parse(res);

        console.log(user);

        window.localStorage.setItem("usr_token", user.private.token);
        window.localStorage.setItem("usr_name", user.public.username);
        window.localStorage.setItem("usr_id", user.public.id);
        window.location.pathname = "/chat";
    }
}