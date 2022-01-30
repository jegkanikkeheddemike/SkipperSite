let messages_url = window.location.href + "/api/messages";
var messages;
$.getJSON(messages_url, (data) => {
    messages = data;
    messages.forEach(element => {
        console.log(element.message_content);
    });
});

