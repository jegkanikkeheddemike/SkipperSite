type Message = {
    text: string;
    id: number;
    user_id: number;
    timestamp: number;
}

type User = {
    client: {
        public: {
            username: string;
            id: number;
            friends: number[];
        };
        private: {
            token: string;
        };
    };
    secret: {
        password: string;
        salt: string;
    }
}

type Chat = {
    chat_name: string;
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