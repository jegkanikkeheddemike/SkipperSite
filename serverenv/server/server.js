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
    let success = false;

    let db = get_database();
    db.users.forEach(user => {
        let hashed_pass = crypto.createHash('sha256').update(cred.pswd + user.secret.pass_salt).digest('hex');
        if (user.client_side.public.username == cred.usrnme && user.secret.password == hashed_pass) {
            success = true;
            res.send(JSON.stringify(user.client_side));
            console.log("LOGGED IN");
        }
    });
    console.log("FAILED LOG IN");
    if (!success)
        res.send("LOGIN_ERR");
})

app.post('/chat/api/users', (req, res) => {
    let cred = req.body;
    console.log(cred);
    if (cred.pswd != cred.c_pswd) {
        res.send("ERR: Passwords dont match");
    }

    let db = get_database();
    //check if a user with the same username exists!
    db.users.forEach(user => {
        if (user.client_side.public.username == cred.usrnme) {
            res.send("ERR: Username already exists");
        }
    });

    let pass_salt = make_salt(255);
    let salted_pass = cred.pswd + pass_salt;

    let hashed_pass = crypto.createHash('sha256').update(salted_pass).digest('hex');
    console.log("hashed pass: " + hashed_pass);

    let token_salt = make_salt(255);
    let token = crypto.createHash('sha256').update(cred.usrnme + token_salt).digest('hex');
    let id = db.next_user_id;
    db.next_user_id++;
    let new_user = {
        client_side: {
            public: {
                id: id,
                username: cred.usrnme
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
    save_database(db);
    res.send(JSON.stringify(new_user.client_side));

})

app.get('/chat', (req, res) => {
    res.sendFile(__dirname + "/public/Chat.html");
})

function get_database() {
    if (!fs.existsSync("./serverenv/chatdb.json")) {
        fs.writeFileSync("./serverenv/chatdb.json", JSON.stringify(
            {
                users: [],
                messages: []
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


app.get('/chat/api/messages', (req, res) => {
    res.send(get_database().messages)
})
app.get('/chat/api/users/:userid', (req, res) => {
    let userid = req.params.userid;
    res.send(get_database().users[userid].client_side.public);
})

app.post('/chat/api/messages', (req, res) => {

    let body = req.body;
    let date = new Date()
    let message = {
        sender: body.userid,
        message_content: body.message_content,
        timestamp: date.getFullYear() + "-" + date.getMonth() + "-" + date.getDate() + " " + date.getHours() + ":" + date.getMinutes()
    }
    let db = get_database();
    db.messages.push(message);
    save_database(db);
    res.sendStatus(200)
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