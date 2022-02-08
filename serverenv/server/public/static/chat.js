var chats_url = window.location.href + "/api/chats/" + localStorage.getItem("usr_id") + "/" + localStorage.getItem("usr_token");
var app = new Vue({
    el: '#app',
    data: {
        chats: [],
        chat_id: -1,
        uid: localStorage.getItem("usr_id"),
        messages: [],
        creating_new_chat: true, //will be set false 
        members_in_chat: [],
        member_options_mode: "none"
    },
    methods: {
        get_chat: chat_id => {
            let chat;

            app.chats.forEach(this_chat => {
                if (this_chat.chat_id == chat_id) {
                    chat = this_chat;
                }
            });
            return chat;
        }
    }
});

function update_chats() {
    console.log(chats_url)
    $.getJSON(chats_url, (data) => {
        app.chats = data;
        app.chat_id = 0;
        update_messages();
        update_chat_members();
    });
}
update_chats();

function update_chat_members() {
    app.members_in_chat = [];



    app.get_chat(app.chat_id).member_ids.forEach(member_id => {
        $.getJSON("/chat/api/users/" + member_id, (data) => {
            let member = {
                member_id: member_id,
                username: data.username
            }
            app.members_in_chat.push(member);
        });
    });
}

async function update_messages() {
    let messages_url = window.location.href + "/api/messages/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;
    $.getJSON(messages_url, (data) => {

        if (app.messages.length != data.length) {
            app.messages = data;
            Vue.nextTick(() => {
                chat.scrollTo(0, chat.scrollHeight);
            });
        }

    });
}
update_messages();


async function update_loop() {
    while (true) {
        await sleep(2000);
        update_messages();
    }
}

update_loop();

function post_message() {
    let messages_url = window.location.href + "/api/messages/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;
    let formdata = JSON.stringify({
        userid: app.uid,
        message_content: chat_input_field.value
    });

    console.log(formdata);

    $.ajax(
        {
            type: "POST",
            url: messages_url,
            data: formdata,
            dataType: "json",
            contentType: "application/json"
        }
    );
    chat_input_field.value = "";
    update_messages();
    chat.scrollTo(0, chat.scrollHeight);
}

function switch_create_chat() {
    app.creating_new_chat = !app.creating_new_chat;

    if (app.creating_new_chat) {
        Vue.nextTick(() => {
            chat_form.action = "chat/api/chats/" + app.uid + "/" + localStorage.getItem("usr_token");
            console.log("Helo " + chat_form.action);

            //Bind ajax_forms to the form
            //MUST be loaded after the DOM
            $("#chat_form").ajaxForm(function (response) {
                new_chat_res(response);
            });
        });
    }
}

app.creating_new_chat = false;

function new_chat_res(res) {
    if (res == "Success") {
        update_chats();
    }
}

function change_chat(chat) {
    app.chats.forEach(app_chat => {
        if (app_chat.chat_id == chat.attributes.chat_id.value) {
            app.chat_id = app_chat.chat_id;
            update_messages();
            update_chat_members();
        }
    });
}

function set_member_options_mode(new_mode) {

    if (app.member_options_mode == new_mode) {
        app.member_options_mode = "none";
    } else {
        app.member_options_mode = new_mode;
    }

    if (new_mode == "invite") {
        Vue.nextTick(() => {
            member_invite_form.action = "chat/api/chats/" + app.chat_id + "/members/" + app.uid + "/" + localStorage.getItem("usr_token");
            //Bind ajax_forms to the form
            //MUST be loaded after the DOM
            $("#member_invite_form").ajaxForm(function (response) {
                if (response == "User added to chat") {
                    update_chat_members();
                } else {
                    console.log(response);
                }
            });
        });
    }
}