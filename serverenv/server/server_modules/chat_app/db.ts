import * as db_types from "./db_types";

import fs from "fs"

var data: db_types.Database;

function db_init() {
    //check if db exists. If not create new.
    if (fs.existsSync("./serverenv/chatdb.json")) {
        data = JSON.parse(fs.readFileSync("./serverenv/chatdb.json").toString());
        console.log("Loaded BD");
    } else {
        console.log("Creating db");
        data = new_database();
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
        user_id: user_id
    }
    data.next_message_id = data.next_message_id + 1;
    data.messages.push(message);
    data.chats[chat_id].member_ids.push(message.id);

    save_db();
    return message.id;
}

function create_user() {
    let user:db_types.User = {
        client: {
            public: {
                username: undefined,
                id: 0,
                friends: []
            },
            private: {
                token: undefined
            }
        },
        secret: {
            password: undefined,
            salt: undefined
        }
    }
}

function new_database(): db_types.Database {
    let db: db_types.Database = {
        messages: [],
        chats: [],
        users: [],
        next_user_id: 0,
        next_message_id: 0,
        next_chat_id: 0
    }
    return db;
}


export { data, db_init, create_message };