type Message = {
    text: String;
    id: number;
    user_id: number;
}

type User = {
    client: {
        public: {
            username: String;
            id: number;
            friends: number[];
        };
        private: {
            token: String;
        };
    };
    secret: {
        password: String;
        salt: String;
    }
}

type Chat = {
    chat_name: String;
    id: number;
    member_ids: number[];
    message_ids: number[];
}

type Database = {
    messages: Message[];
    chats: Chat[];
    users: User[];

    next_user_id: number;
    next_message_id: number;
    next_chat_id: number;
}

export { Database, User, Message, Chat }