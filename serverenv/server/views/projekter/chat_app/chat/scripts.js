if (!logged_in()) {
    location.pathname = "/projekter/chat_app/login/"
}

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

            if (app) {
                app.chats.forEach(this_chat => {
                    if (this_chat.chat_id == chat_id) {
                        chat = this_chat;
                    }
                });
                return chat;
            }
            return "Loading";

        }
    }
});