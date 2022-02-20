import * as db_types from "./db_types";
import fs from "fs"
import * as crypto from "crypto";

var data: db_types.Database;

function db_init() {
    //check if db exists. If not create new.
    if (fs.existsSync("./serverenv/chatdb.json")) {
        data = JSON.parse(fs.readFileSync("./serverenv/chatdb.json").toString());
        console.log("Loaded BD");
    } else {
        console.log("Creating db");
        new_database();
        save_db();
    }
}

function save_db() {
    fs.writeFileSync("./serverenv/chatdb.json", JSON.stringify(data));

}

function create_message(text: string, user_id: number, chat_id: number): number {
    let message: db_types.Message = {
        text: text,
        id: data.next_message_id,
        user_id: user_id,
        timestamp: new Date().getTime()
    }
    data.next_message_id = data.next_message_id + 1;
    data.messages.push(message);
    data.chats[chat_id].message_ids.push(message.id);

    save_db();
    return message.id;
}

function create_chat(chat_name: string, creator_id: number): number {
    let chat: db_types.Chat = {
        chat_name: chat_name,
        id: data.next_chat_id,
        member_ids: [creator_id],
        message_ids: []
    }
    data.next_chat_id = data.next_chat_id + 1;
    data.chats.push(chat);
    save_db();
    return chat.id;
}

function create_user(username: string, password: string): number {
    let salt = random_salt();
    let user: db_types.User = {
        client: {
            public: {
                username: username,
                id: data.next_user_id,
                friends: []
            },
            private: {
                token: "RANDOMTOKEN"
            }
        },
        secret: {
            password: hash(password, salt),
            salt: salt,
        }
    }
    data.next_user_id = data.next_user_id + 1;
    data.users.push(user);
    data.chats[0].member_ids.push(user.client.public.id);
    save_db();
    return user.client.public.id;
}

function new_database() {
    let db: db_types.Database = {
        messages: [],
        chats: [],
        users: [],
        next_user_id: 0,
        next_message_id: 0,
        next_chat_id: 0
    }
    data = db;
    create_chat("Global chat", 0);
    create_user("Admin", "123");
    
    //remove duplicate 0 from globalchat.member_ids
    data.chats[0].member_ids = [0];
}

function hash(password: string, salt: string) {
    let ps = password + "" + salt;

    return crypto.createHash('sha256').update(ps).digest('hex');
}

function random_salt(): string {

    let result = '';
    let characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let charactersLength = characters.length;
    for (let i = 0; i < 64; i++) {
        result += characters.charAt(Math.floor(Math.random() *
            charactersLength));
    }
    return result;

}

export { data, db_init, create_message, hash, create_chat, create_user };