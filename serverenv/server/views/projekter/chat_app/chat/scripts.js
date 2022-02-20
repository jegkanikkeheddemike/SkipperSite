if (!logged_in()) {
    location.pathname = "/projekter/chat_app/login/"
}

var app = new Vue({
    el: '#app',
    data: {
        chats: [],
        chat_id: 0,
        uid: localStorage.getItem("usr_id"),
        messages: [],
        creating_new_chat: false,
        members_in_chat: [],
        member_options_mode: "none"
    },
    methods: {
        get_chat: chat_id => {

            if (app) {
                if (app.chat_id == -1) {
                    return "No chat selected";
                }
                for (let i = 0; i < app.chats.length; i++) {
                    if (app.chats[i].id == chat_id) {
                        return app.chats[i];
                    }
                }
            }
            return "Loading";

        }
    }
});


function handle(res, success, failure) {
    if (res.success) {
        success(res.data)
    } else {
        if (res.panic) {
            alert(res.data);
        } else {
            console.log(res.data);
        }
        if (res.data == "VERIFICATION_ERROR") {
            alert("Failed to verify user, logging out");
            log_out();
        } else {
            failure(res.data);
        }
    }
}

async function load_chats() {
    let url = "/projekter/chat_app/chat/chats/" + app.uid + "/" + localStorage.getItem("usr_token");
    $.getJSON(url, res => {
        handle(res, data => {
            app.chats = data;
            if (data.length != 0) {
                if (app.chat_id == -1) {
                    change_chat(0);
                }
            }
        }, data => { });
    })
}
load_chats();
setInterval(load_chats, 5000);

async function load_messages() {
    let url = "/projekter/chat_app/chat/messages/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;
    $.getJSON(url, res => {
        handle(res, data => {
            app.messages = data;
        }, _data => { });
    });
}
load_messages();
setInterval(load_messages, 2000);

function change_chat(chat) {

    let chat_id;
    if (typeof (chat) == "number") {
        chat_id = chat;
    } else {
        chat_id = parseInt(chat.attributes.getNamedItem("chat_id").value);
    }
    app.chat_id = chat_id;

    message_form.action = "messages/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;
    load_messages();
    load_members();
    app.member_options_mode = "none";
}
//Bind ajax_forms to the form
//MUST be loaded after the DOM
$("#message_form").ajaxForm(function (res) {
    handle(res, _data => {
        chat_input_field.value = "";
        load_messages();
    }, data => {
        alert("Failed to send message. Error: " + data);
    })
});
message_form.action = "messages/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;

function switch_create_chat() {
    app.creating_new_chat = !app.creating_new_chat;

    if (app.creating_new_chat) {
        Vue.nextTick(() => {
            create_chat_form.action = "/projekter/chat_app/chat/chats/" + app.uid + "/" + localStorage.getItem("usr_token");
            $("#create_chat_form").ajaxForm(function (res) {
                handle(res, data => {
                    switch_create_chat();
                    load_chats();
                    change_chat(data);
                }, data => {
                    alert("Failed to create chat. Error: " + data);
                })
            });
        });

    }
}

function set_member_options_mode(mode) {
    app.member_options_mode = mode;
    if (mode == "invite") {
        Vue.nextTick(() => {
            member_invite_form.action = "/projekter/chat_app/chat/members/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;
            $("#member_invite_form").ajaxForm(function (res) {
                handle(res, data => {
                    load_members();
                }, data => { })
            });
        });
    }
}

async function load_members() {
    let url = "/projekter/chat_app/chat/members/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;
    $.getJSON(url, res => {
        handle(res, data => {
            app.members_in_chat = data;
        }, _data => { });
    })
}

load_members();
setInterval(load_members, 5000);