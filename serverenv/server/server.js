const port = 3000;
const express = require('express')
const app = express()
const fs = require("fs")
const os = require('os')

app.use(express.static('serverenv/server/public'))

var visists_this_up = 0;
var up_start = Date.now();

app.get('/', (req, res) => {
    const buffer = fs.readFileSync("serverenv/server/public/Homepage.html");
    let fileContent = buffer.toString();
    fileContent = fileContent.replace("{hostname}",os.hostname());

    fileContent = fileContent.replace("{uptime}",Math.floor((Date.now()-up_start)/60000));

    visists_this_up++;
    fileContent = fileContent.replace("{visits}",visists_this_up);
    
    console.log("Site accesed from: " + (req.headers['x-forwarded-for'] || req.socket.remoteAddress ) + " at " + get_now());
    
    res.send(fileContent);
})

app.listen(port, () => {
    console.log(`    Server listening at http://localhost:${port}`)
})

function get_now() {
    let date = new Date();
    return date.toDateString() + " " + date.getHours() + ":" + date.getMinutes();
}