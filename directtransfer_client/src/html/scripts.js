
var socket = null;
var locked = false;

function setup(port) {

    socket = new WebSocket('ws://127.0.0.1:' + port);

    // Connection opened
    socket.addEventListener('open', function (event) {
        log("Hello server <3")
    });


    socket.addEventListener('message', function (event) {
        if (typeof (event.data) === "string") {
            if (event.data.substring(0, 5) == "PATH:") {
                filefoldername.value = (event.data + "").substring(5);
            } else if (event.data.substring(0, 8) == "STATUS: ") {
                app.status = event.data.substring(8);
                if (app.status == "Transfer complete" || app.status.substring(0, 6) == "ERROR:") {
                    locked = false;
                }

            } else if (event.data.substring(0, 10) == "PROGRESS: ") {
                app.progress = event.data.substring(10);

            } else if (event.data.substring(0, 13) == "listening on ") {
                app.host_ip = event.data.substring(13);
            } else if (event.data.substring(0, 7) == "files: ") {
                let size_index = event.data.search("total size: ");
                app.files = event.data.substring(7, size_index);
                app.totalsize = event.data.substring(size_index + 12);

            } else {
                log("FAILED TO UNDERSTAND COMMAND " + event.data)
            }
        } else {
            log("NOT A STRING: " + event.data + " its " + typeof (event.data));
        }
    });
}

function log(text) {
    socket.send("LOG: " + text);
}

function change_mode() {
    if (locked)
        return;

    if (app.mode == "host") {
        app.mode = "leech";
    } else if (app.mode == "leech") {
        app.mode = "host";
    } else {
        log("UNKNOWN MODE: " + app.mode);
    }
}

function pick_folder() {
    if (locked)
        return;
    command("PICK_FOLDER");
}

function begin_mode() {
    if (locked)
        return;

    locked = true;

    if (app.mode == "host") {
        command("HOST " + filefoldername.value);
        app.progess = "0%";
    } else if (app.mode = "leech") {

        let ip = document.getElementById("connection_ip").value;
        ip = ip.trim();
        command("LEECH " + ip);
        app.progess = "0%";
    }
}

function command(command) {
    socket.send("CMD: " + command);
}