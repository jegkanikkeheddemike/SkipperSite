let messages_url = window.location.href + "/api/messages";
console.log(messages_url);
var messages;
$.getJSON(messages_url, (data) => {
    messages = data;

    console.log("LENGTH: " + messages.length);

    messages.forEach(element => {
        console.log("LOG")
        console.log(element.message_content);
    });
});

