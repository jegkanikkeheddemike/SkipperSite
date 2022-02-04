const port = 3000;
const express = require('express')
const app = express()
const bodyParser = require('body-parser')
const crypto = require("crypto")
const fs = require("fs")
const os = require('os')

app.use(express.static('serverenv/server/public'))
app.use(express.json());
app.use(bodyParser.urlencoded({     // to support URL-encoded bodies
    extended: true
}));

var visists_this_up = 0;
var up_start = Date.now();

app.get('/', (req, res) => {
    const buffer = fs.readFileSync("serverenv/server/public/Homepage.html");
    let fileContent = buffer.toString();
    fileContent = fileContent.replace("{hostname}", os.hostname());

    fileContent = fileContent.replace("{uptime}", Math.floor((Date.now() - up_start) / 60000));

    visists_this_up++;
    fileContent = fileContent.replace("{visits}", visists_this_up);

    console.log("Site accesed from: " + (req.headers['x-forwarded-for'] || req.socket.remoteAddress) + " at " + get_now());

    res.send(fileContent);
})

app.listen(port, () => {
    console.log(`    Server listening at on ${port}`)
})

function get_now() {
    let date = new Date();
    return date.toDateString() + " " + date.getHours() + ":" + date.getMinutes();
}

//DATABASE AND MESSAGE APP
app.get('/login', (req, res) => {
    res.sendFile(__dirname + "/public/Login.html");
})

app.post('/login', (req, res) => {
    let cred = req.body;

    let db = get_database();
    let success = false;
    db.users.forEach(user => {
        let hashed_pass = crypto.createHash('sha256').update(cred.pswd + user.secret.pass_salt).digest('hex');
        if (user.client_side.public.username == cred.usrnme && user.secret.password == hashed_pass) {
            res.send(user.client_side);
            success = true;
        }
    });
    if (!success)
        res.send("LOGIN_ERR");
})

app.post('/chat/api/users', (req, res) => {
    let cred = req.body;
    if (cred.pswd != cred.c_pswd) {
        res.send("ERROR: Passwords dont match");
        return;
    }

    let db = get_database();
    //check if a user with the same username exists!
    let failure = false;
    db.users.forEach(user => {
        if (user.client_side.public.username == cred.usrnme) {
            res.send("ERROR: Username already exists");
            failure = true;
        }
    });
    if (failure)
        return;

    let pass_salt = make_salt(64);
    let salted_pass = cred.pswd + pass_salt;

    let hashed_pass = crypto.createHash('sha256').update(salted_pass).digest('hex');
    console.log("hashed pass: " + hashed_pass);

    let token_salt = make_salt(64);
    let token = crypto.createHash('sha256').update(cred.usrnme + token_salt).digest('hex');
    let id = db.next_user_id;
    db.next_user_id++;
    let new_user = {
        client_side: {
            public: {
                id: id,
                username: cred.usrnme,
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
    db.users.push(new_user);
    db.chats[0].member_ids.push(id);
    save_database(db);
    res.send(new_user.client_side);
})

app.get('/chat', (req, res) => {
    res.sendFile(__dirname + "/public/Chat.html");
})

function get_database() {
    if (!fs.existsSync("./serverenv/chatdb.json")) {
        console.log("DB DOES NOT EXIST, CREATING!");
        fs.writeFileSync("./serverenv/chatdb.json", JSON.stringify(
            {
                next_user_id: 1,
                next_message_id: 1,
                next_chat_id: 1,
                users: [
                    {
                        client_side: {
                            public: {
                                id: 0,
                                username: "example_user",
                                friends: []
                            },
                            private: {
                                token: "02937286a97f8450b18ad121e7aafc91a7ded51f4188f4b3f40f704ba67ab316"
                            }
                        },
                        secret: {
                            password: "8c32bc145e80f78b7a39f16fe838fd4dad36e0865f9a5c284a4049ee27da2cb7",
                            pass_salt: "qrWtj1yCSyLFTTU6zsOrziHE7KTVrwj4L90s6EFjJxfrBNTeEe2aosid8NoBWig8"
                        }
                    }
                ],
                chats: [
                    {
                        chat_id: 0,
                        chat_name: "example chat",
                        member_ids: [
                            0
                        ],
                        message_ids: [0]
                    }
                ],
                messages: [
                    {
                        message_id: 0,
                        message_content: "example_message",
                        user_id: 0
                    }
                ]
            }
        ));
    }
    return JSON.parse(fs.readFileSync("./serverenv/chatdb.json").toString());
}

function save_database(database) {
    if (database != null) {
        fs.writeFileSync("./serverenv/chatdb.json", JSON.stringify(database));
    }
}


app.get('/chat/api/messages/:user_id/:token/:chat_id', (req, res) => {
    let chat_messages = [];
    let user_id = req.params.user_id;
    let token = req.params.token;
    let chat_id = req.params.chat_id

    let db = get_database();
    //first check if token is valid

    if (user_id < 0 || chat_id < 0) {
        res.send("Invalid creds");
        return;
    }

    if (db.users[user_id].client_side.private.token == token) {
        if (db.chats[chat_id].member_ids.includes(parseInt(user_id))) {
            //if token and user is valid for the chat, find all messages in the chat

            db.chats[chat_id].message_ids.forEach(message_id => {
                let message = db.messages[message_id];
                message.username = db.users[message.user_id].client_side.public.username;
                chat_messages.push(message);
            });
            res.send(chat_messages);
        }
    } else {
        res.send("invalid token verification")
    }
})

app.get('/chat/api/users/:user_id', (req, res) => {
    let user_id = req.params.user_id;
    res.send(get_database().users[user_id].client_side.public);
})

app.post('/chat/api/messages/:user_id/:token/:chat_id', (req, res) => {

    let user_id = req.params.user_id;
    let token = req.params.token;
    let chat_id = req.params.chat_id;

    //first check if the sender has permission
    if (user_id < 0 || chat_id < 0) {
        console.log(1);
        res.send("Invalid userid or chat id")
        return;
    }

    let db = get_database();

    if (db.users[parseInt(user_id)].client_side.private.token != token) {
        console.log(2)
        res.send("Invalid token");
        return;
    }

    if (!db.chats[parseInt(chat_id)].member_ids.includes(parseInt(user_id))) {
        console.log(3);
        res.send("User does not have access to chat");
        return;
    }

    let body = req.body;
    let date = new Date()

    let message = {
        user_id: body.userid,
        message_content: body.message_content,
        timestamp: date.getFullYear() + "-" + date.getMonth() + "-" + date.getDate() + " " + date.getHours() + ":" + date.getMinutes(),
        message_id: db.next_message_id
    }
    db.next_message_id++;

    db.messages.push(message);
    db.chats[chat_id].message_ids.push(message.message_id);
    save_database(db);
    res.sendStatus(200)
})

app.get('/chat/api/chats/:userid/:token', (req, res) => {
    let user_id = req.params.userid;
    let token = req.params.token;

    let available_chats = [];

    let db = get_database();
    db.chats.forEach(chat => {
        if (chat.member_ids.includes(parseInt(user_id))) {
            if (db.users[user_id].client_side.private.token == token) {
                available_chats.push(chat);
            }
        }
    });
    res.send(available_chats);
})


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