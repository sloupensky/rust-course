# DATA UTILS
Its helper library for manipulating data in database

* `get_user_by_name(user_name: String) -> Result<User, diesel::result::Error>` return file or raise error when no user found
* `insert_message(text: String, file: String, image: String) -> Result<Message, diesel::result::Error>` insert message into database