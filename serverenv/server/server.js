const port = 3000;
const express = require('express')
const e_app = express()
const bodyParser = require('body-parser')
const fs = require("fs");
const os = require('os');

e_app.use(express.static('serverenv/server/public'))
e_app.use(express.json());
e_app.use(bodyParser.urlencoded({     // to support URL-encoded bodies
    extended: true
}));

require("./chat")(e_app);

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
