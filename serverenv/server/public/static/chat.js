var messages_url = window.location.href + "/api/messages";

var app = new Vue({
    el: '#app',
    data: {
        messages: [],
        chat_name: "No chat loaded",
        uid: 0
    }
});

function update_messages() {
    $.getJSON(messages_url, (data) => {

        app.messages = [];

        data.forEach(element => {
            app.messages.push(element);
        });
    });
}
update_messages();

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function update_loop() {
    while (true) {
        await sleep(1000);
        update_messages();
    }
}

update_loop();

function post_message() {
    let formdata = JSON.stringify({
        userid: 0,
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
    chat.scrollTo(0,chat.scrollHeight);
}