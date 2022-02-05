const fs = require("fs");
const crypto = require("crypto");

var data = {};

function load_database() {
    if (!fs.existsSync("./serverenv/chatdb.json")) {
        console.log("DB DOES NOT EXIST, CREATING!");
        data = new_db();
        new_chat("Global chat");
        new_user("Example user", "123");
        new_message("Example message", 0, 0);
        save_database();
    } else {
        data = JSON.parse(fs.readFileSync("./serverenv/chatdb.json").toString());
    }
}
load_database();


module.exports = {
    data: data,
    new_user: new_user,
    new_chat: new_chat,
    new_message: new_message
}

function save_database() {
    fs.writeFileSync("./serverenv/chatdb.json", JSON.stringify(data));
}

function new_user(username, password) {
    let pass_salt = make_salt(64);
    let salted_pass = password + pass_salt;

    let hashed_pass = crypto.createHash('sha256').update(salted_pass).digest('hex');

    let token_salt = make_salt(64);
    let token = crypto.createHash('sha256').update(username + token_salt).digest('hex');

    let new_user = {
        client_side: {
            public: {
                id: data.next_user_id,
                username: username,
                friends: []
            },
            private: {
                token: token
            }
        },
        secret: {
            password: hashed_pass,
            pass_salt: pass_salt
        }
    }
    data.next_user_id++;
    data.users.push(new_user);
    data.chats[0].member_ids.push(new_user.client_side.public.id);
    save_database();
    return new_user.client_side.public.id;
}

function new_message (message_content, user_id, chat_id) {
    let date = new Date();
    let timestamp = date.getFullYear() + "-" + date.getMonth() + "-" + date.getDate() + " " + date.getHours() + ":" + date.getMinutes();
    let message = {
        message_id: data.next_message_id,
        message_content: message_content,
        user_id: user_id,
        username: data.users[user_id].username,
        timestamp: timestamp
    }
    data.next_message_id++;
    data.messages.push(message);
    data.chats[chat_id].message_ids.push(message.message_id);
    save_database();
    return message.message_id;
}

function new_chat(chat_name) {
    let chat = {
        chat_id: data.next_chat_id,
        chat_name: chat_name,
        member_ids: [],
        message_ids: []
    }
    data.next_chat_id++;
    data.chats.push(chat);
    save_database();
    return chat.chat_id;
}

function new_db() {
    let db = {
        next_user_id: 0,
        next_message_id: 0,
        next_chat_id: 0,
        users: [],
        chats: [],
        messages: []
    }
    return db;
}


function make_salt(length) {
    var result = '';
    var characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for (var i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() *
            charactersLength));
    }
    return result;
}