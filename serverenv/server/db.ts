const fs = require("fs");
const { type } = require("os");

exports.get_database = () => {
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
                        chat_name: "global chat",
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

exports.save_database = database => {
    if (database != null) {
        fs.writeFileSync("./serverenv/chatdb.json", JSON.stringify(database));
    }
}