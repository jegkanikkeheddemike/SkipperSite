import bodyParser from "body-parser";
import express from "express";
import * as db from "../db"
import * as db_types from "../db_types";

const error = {
    verification_error: { success: false, panic: true, data: "VERIFICATION_ERROR" },
    invalid_userid_error: { success: false, panic: false, data: "INVALID_USER_ERROR" },
    user_already_in_chat_error: { success: false, panic: true, data: "This user is already in the chat" }
}
function custom_error(data: any, panic: boolean) {
    return { success: false, panic, data }
}

function success(data: any) {
    return {
        success: true,
        data: data
    }
}

function chat_init(e_app: express.Express) {
    //Verify logged in
    e_app.get('/projekter/chat_app/verify/:user_id/:token', (req, res) => {
        let user_id = parseInt(req.params.user_id);
        let token = req.params.token;

        if (db.data.users[user_id].client.private.token == token) {
            res.send(success({}));
        } else {
            res.send(error.verification_error);
        }
    });

    //post message to chat
    e_app.post('/projekter/chat_app/chat/messages/:user_id/:token/:chat_id', (req, res) => {
        let user_id = parseInt(req.params.user_id);
        let token = req.params.token;
        let chat_id = parseInt(req.params.chat_id);

        //First, verify user
        if (db.data.users[user_id].client.private.token != token) {
            res.send(error.verification_error);
            return;
        }

        //check if user is in the chat
        if (!db.data.chats[chat_id].member_ids.includes(user_id)) {
            res.send(custom_error("USER NOT IN CHAT", true));
            return;
        }

        db.create_message(req.body.message_text, user_id, chat_id);

        res.send(success("Message sent"));
    });

    //get messages of chat
    e_app.get('/projekter/chat_app/chat/messages/:user_id/:token/:chat_id', (req, res) => {
        let user_id = parseInt(req.params.user_id);
        let token = req.params.token;
        let chat_id = parseInt(req.params.chat_id);

        if (user_id < 0) {
            res.send(error.invalid_userid_error);
            return;
        }
        if (chat_id < 0) {
            res.send(custom_error("Invalid chat id", false));
            return;
        }

        //First, verify user
        if (db.data.users[user_id].client.private.token != token) {
            res.send(error.verification_error);
            return;
        }

        //check if user is in the chat
        if (!db.data.chats[chat_id].member_ids.includes(user_id)) {
            res.send(custom_error("USER NOT IN CHAT", true));
            return;
        }

        type client_message = {
            inner_message: db_types.Message;
            username: string;
            date: string;
        }

        let messages: client_message[] = [];
        //send messages
        for (let i = 0; i < db.data.chats[chat_id].message_ids.length; i++) {
            let m_id = db.data.chats[chat_id].message_ids[i];
            let message = db.data.messages[m_id];
            let timestamp = new Date(message.timestamp);
            messages.push({
                inner_message: message,
                username: db.data.users[message.user_id].client.public.username,
                date: timestamp.getDate() + "-" + (timestamp.getMonth() + 1) + "-" + timestamp.getFullYear() + " " + timestamp.getHours() + ":" + timestamp.getMinutes()
            });
        }
        res.send(success(messages));
    });

    //get chats
    e_app.get('/projekter/chat_app/chat/chats/:user_id/:token', (req, res) => {
        let user_id = parseInt(req.params.user_id);
        let token = req.params.token;
        let chats: db_types.Chat[] = [];


        //first, verify the user
        if (db.data.users[user_id].client.private.token != token) {
            res.send(error.verification_error);
            return;
        }

        //find all chats of the user
        for (let i = 0; i < db.data.chats.length; i++) {
            let chat = db.data.chats[i];
            if (chat.member_ids.includes(user_id)) {
                chats.push(chat);
            }
        }
        //reverse chats so newer get on top, then sort by newest message
        chats.reverse();

        chats.sort((a, b) => {
            if (a.message_ids.length == 0 && b.message_ids.length == 0) {
                return 0;
            } else {
                if (a.message_ids.length == 0) {
                    return 1;
                }
                if (b.message_ids.length == 0) {
                    return -1;
                }
            }
            let a_message_id = a.message_ids[a.message_ids.length - 1];
            let b_message_id = b.message_ids[b.message_ids.length - 1];
            let a_time = db.data.messages[a_message_id].timestamp;
            let b_time = db.data.messages[b_message_id].timestamp;
            return (b_time - a_time);

        });

        //Respond with chats
        res.send(success(chats));
    });


    e_app.post('/projekter/chat_app/chat/chats/:user_id/:token', (req, res) => {
        let user_id = parseInt(req.params.user_id);
        let token = req.params.token;

        if (user_id < 0) {
            res.send(error.invalid_userid_error);
            return;
        }

        //Verify user
        if (db.data.users[user_id].client.private.token != token) {
            res.send(error.verification_error);
            return;
        }
        let chat_id = db.create_chat(req.body.chat_name, user_id);
        res.send(success(chat_id));
    });

    //get chat members
    e_app.get('/projekter/chat_app/chat/members/:user_id/:token/:chat_id', (req, res) => {
        let user_id = parseInt(req.params.user_id);
        let token = req.params.token;
        let chat_id = parseInt(req.params.chat_id);

        if (user_id < 0) {
            res.send(error.invalid_userid_error);
            return;
        }
        if (chat_id < 0) {
            res.send(custom_error("Invalid chat id", false));
            return;
        }

        //First, verify user
        if (db.data.users[user_id].client.private.token != token) {
            res.send(error.verification_error);
            return;
        }

        //check if user is not in chat
        if (!db.data.chats[chat_id].member_ids.includes(user_id)) {
            res.send(error.invalid_userid_error);
            return;
        }

        let members_public = [];

        for (let i = 0; i < db.data.chats[chat_id].member_ids.length; i++) {
            let u_id = db.data.chats[chat_id].member_ids[i];
            let public_data = db.data.users[u_id].client.public;
            members_public.push(public_data);
        }

        res.send(success(members_public));
    });

    //add member to chat
    e_app.post('/projekter/chat_app/chat/members/:user_id/:token/:chat_id', (req, res) => {
        let user_id = parseInt(req.params.user_id);
        let token = req.params.token;
        let chat_id = parseInt(req.params.chat_id);

        if (user_id < 0) {
            res.send(error.invalid_userid_error);
            return;
        }
        if (chat_id < 0) {
            res.send(custom_error("Invalid chat id", false));
            return;
        }

        //First, verify user
        if (db.data.users[user_id].client.private.token != token) {
            res.send(error.verification_error);
            return;
        }

        //check if user already is in the chat
        for (let i = 0; i < db.data.chats[chat_id].member_ids.length; i++) {
            let chat_user_id = db.data.chats[chat_id].member_ids[i];

            let username = db.data.users[chat_user_id].client.public.username;
            if (username == req.body.username) {
                res.send(error.user_already_in_chat_error);
                return;
            }
        }

        //if the user is not in the chat, the find the user_id and add it to the chat
        for (let i = 0; i < db.data.users.length; i++) {
            if (db.data.users[i].client.public.username == req.body.username) {
                db.data.chats[chat_id].member_ids.push(i);
                res.send(success("Member added to chat"));
                return;
            }
        }

        res.send(custom_error("User does not exist", true));
    });
}


export { chat_init }