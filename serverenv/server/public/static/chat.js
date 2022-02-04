var chats_url = window.location.href + "/api/chats/" + localStorage.getItem("usr_id") + "/" + localStorage.getItem("usr_token");

var app = new Vue({
    el: '#app',
    data: {
        chats: [],
        chat_id: -1,
        uid: localStorage.getItem("usr_id"),
        messages: []
    },
    methods: {
        get_user: async function (user_id) {
            let username = "";
            let target_url = location.href + "/api/users/" + user_id;

            
            $.getJSON(target_url, (data) => {
                username = data.username;
            });
        }
    }
});

function update_chats() {
    $.getJSON(chats_url, (data) => {
        app.chats = data;
        if (data.length != 0) {
            app.chat_id = 0;
        }
        update_messages();
    });

}
update_chats();
console.log(app.chats);

function update_messages() {
    let messages_url = window.location.href + "/api/messages/" + app.uid + "/" + localStorage.getItem("usr_token") + "/" + app.chat_id;
    $.getJSON(messages_url, (data) => {

        app.messages = data;
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