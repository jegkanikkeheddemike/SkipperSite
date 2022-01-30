const port = 3000;
const express = require('express')
const app = express()
const fs = require("fs")
const os = require('os')

app.use(express.static('serverenv/server/public'))
app.use(express.json());

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

app.get('/chat',(req,res) => {
    res.sendFile(__dirname+"/public/Chat.html");
})

function get_database() {
    if (!fs.existsSync("./serverenv/server/localdb.json")) {
        fs.writeFileSync("./serverenv/server/localdb.json", JSON.stringify(
            {
                users: [],
                messages: []
            }
        ));
    }
    return JSON.parse(fs.readFileSync("./serverenv/server/localdb.json").toString());
}

function save_database(database) {
    if (database != null) {
        fs.writeFileSync("./serverenv/server/localdb.json", JSON.stringify(database));
    }
}


app.get('/chat/api/messages', (req, res) => {
    res.send(get_database().messages)
})
app.get('/chat/api/users/:userid',(req,res) => {
    let userid = req.params.userid;
    res.send(get_database().users[userid]);
})

app.post('/chat/api/users', (req, res) => {
    let body = req.body
    let db = get_database();
    let user = {
        username: body.username,
        password: body.password,
        userid: db.users.length
    }
    db.users.push(user);
    save_database(db);
    res.sendStatus(200);
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