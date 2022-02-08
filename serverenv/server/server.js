const port = 3000;
const express = require('express')
const e_app = express()
const bodyParser = require('body-parser')
const crypt = require("crypto")
const fs = require("fs");
const os = require('os');
const db = require('./db.js')

e_app.use(express.static('serverenv/server/public'))
e_app.use(express.json());
e_app.use(bodyParser.urlencoded({     // to support URL-encoded bodies
    extended: true
}));

var visists_this_up = 0;
var up_start = Date.now();

e_app.get('/', (req, res) => {
    const buffer = fs.readFileSync("serverenv/server/public/Homepage.html");
    let fileContent = buffer.toString();
    fileContent = fileContent.replace("{hostname}", os.hostname());

    fileContent = fileContent.replace("{uptime}", Math.floor((Date.now() - up_start) / 60000));

    visists_this_up++;
    fileContent = fileContent.replace("{visits}", visists_this_up);

    console.log("Site accesed from: " + (req.headers['x-forwarded-for'] || req.socket.remoteAddress) + " at " + get_now());

    res.send(fileContent);
})

e_app.listen(port, () => {
    console.log(`    Server listening at on ${port}`)
})

function get_now() {
    let date = new Date();
    return date.toDateString() + " " + date.getHours() + ":" + date.getMinutes();
}

//DATABASE AND MESSAGE APP
e_app.get('/login', (req, res) => {
    res.sendFile(__dirname + "/public/Login.html");
})

e_app.post('/login', (req, res) => {
    let cred = req.body;
    let success = false;

    db.data.users.forEach(user => {
        let hashed_pass = crypt.createHash('sha256').update(cred.pswd + user.secret.pass_salt).digest('hex');
        if (user.client_side.public.username == cred.usrnme && user.secret.password == hashed_pass) {
            res.send(user.client_side);
            success = true;
        }
    });
    if (!success)
        res.send("LOGIN_ERR");
})

e_app.post('/chat/api/users', (req, res) => {
    let cred = req.body;
    if (cred.pswd != cred.c_pswd) {
        res.send("ERROR: Passwords dont match");
        return;
    }

    //check if a user with the same username exists!
    let failure = false;
    db.data.users.forEach(user => {
        if (user.client_side.public.username == cred.usrnme) {
            res.send("ERROR: Username already exists");
            failure = true;
        }
    });
    if (failure)
        return;

    let id = db.new_user(cred.usrnme, cred.pswd);
    let new_user = db.data.users[id];
    res.send(new_user.client_side);
})

e_app.get('/chat', (req, res) => {
    res.sendFile(__dirname + "/public/Chat.html");
})




e_app.get('/chat/api/messages/:user_id/:token/:chat_id', (req, res) => {
    let chat_messages = [];
    let user_id = req.params.user_id;
    let token = req.params.token;
    let chat_id = req.params.chat_id

    //first check if token is valid

    if (user_id < 0 || chat_id < 0) {
        res.send("Invalid creds");
        return;
    }

    if (db.data.users[user_id].client_side.private.token == token) {
        if (db.data.chats[chat_id].member_ids.includes(parseInt(user_id))) {
            //if token and user is valid for the chat, find all messages in the chat

            db.data.chats[chat_id].message_ids.forEach(message_id => {
                let message = db.data.messages[message_id];
                message.username = db.data.users[message.user_id].client_side.public.username;
                chat_messages.push(message);
            });
            res.send(chat_messages);
        }
    } else {
        res.send("invalid token verification")
    }
})

e_app.get('/chat/api/users/:user_id', (req, res) => {
    let user_id = req.params.user_id;
    res.send(db.data.users[user_id].client_side.public);
})

e_app.post('/chat/api/messages/:user_id/:token/:chat_id', (req, res) => {

    let user_id = req.params.user_id;
    let token = req.params.token;
    let chat_id = req.params.chat_id;

    //first check if the sender has permission
    if (user_id < 0 || chat_id < 0) {
        res.send("Invalid userid or chat id")
        return;
    }


    if (db.data.users[parseInt(user_id)].client_side.private.token != token) {
        res.send("Invalid token");
        return;
    }

    if (!db.data.chats[parseInt(chat_id)].member_ids.includes(parseInt(user_id))) {
        res.send("User does not have access to chat");
        return;
    }

    let body = req.body;

    db.new_message(body.message_content, user_id, chat_id);
    res.sendStatus(200)
})

e_app.get('/chat/api/chats/:userid/:token', (req, res) => {

    console.log(req.params);

    let user_id = parseInt(req.params.userid);
    let token = req.params.token;
    let available_chats = [];

    if (db.data.users[user_id].client_side.private.token != token) {
        res.send("Token not valid");
        return;
    }


    db.data.chats.forEach(chat => {
        if (chat.member_ids.includes(user_id)) {
            available_chats.push(chat);
        }
    });
    console.log(available_chats);
    res.send(available_chats);
})

e_app.post('/chat/api/chats/:user_id/:token', (req, res) => {
    console.log(req.params);
    console.log(req.body);

    let user_id = req.params.user_id;
    let token = req.params.token;
    let chat_name = req.body.chat_name;

    if (db.data.users[user_id].client_side.private.token != token) {
        res.send("Validication error creating form.")
        return;
    }

    let chat_id = db.new_chat(chat_name);
    console.log("created chat id " + chat_id);
    db.add_user_to_chat(chat_id, user_id);
    res.send("Success");
})


e_app.post("/chat/api/chats/:chat_id/members/:user_id/:user_token", (req,res) => {
    let chat_id = req.params.chat_id;
    let user_id = parseInt(req.params.user_id);
    let token = req.params.user_token;

    //First check if token is valid;
    if (db.data.users[user_id].client_side.private.token != token) {
        res.send("invalid token");
        return;
    }

    //then check if user is in the chat
    if (!db.data.chats[chat_id].member_ids.includes(parseInt(user_id))) {
        res.send("User is not in chat");
        return;
    }

    let invite_user_username = req.body.username;
    db.data.users.forEach(user => {

        //find use with the username
        if (user.client_side.public.username == invite_user_username) {

            //check if the user already is in the chat
            if (db.data.chats[chat_id].member_ids.includes(user.client_side.public.id)){
                res.send("User already in chat");
                return;
            }
            db.data.chats[chat_id].member_ids.push(user.client_side.public.id);
            res.send("User added to chat");
        }
    });
});
