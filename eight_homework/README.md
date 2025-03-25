# Description
This program is used for sending and processing messages of file, image and text. 
It has two modes in which can be started - client and server.

# Setup
You need to install diesel and diesel_cli - `cargo install diesel_cli`, 
then copy .env.dist to .env and set your database, 
you need to have sqlite3 installed (its located in `./data/database.sqlite`), for ubuntu - `sudo apt-get install sqlite3 libsqlite3-dev`
then `diesel migration run`, you will then have test user for using with application.
Migrations are located in `./migrations/` folder.

## Usage

Program can be started in two versions, client and server.
For communication you need to start server first and then client.
For both versions of program (client/server) you need to provide IP on which server/client will be listening for connection.
Program provides logging with `log` crate which outputs colored informations into CLI,
you can control log output by setting `RUST_LOG` env variable (for ex. to debug which is also default).

## Client
After starting client you will be asked to provide server IP and port, default is `localhost:111`,
then you need to provide correct user from database, there is user called test,
then you can select which type of message you want to send
* **image** - you will be asked for path to any image, image will be sent to the server and then processed by clients and saved into `./images/` folder
* **file** - you will be asked for path to any file, this file will be sent through the server and back to all clients and then saved into `./files/` folder
* **text** - you can enter any text and it will be sent to all clients and then write as an output
* **quit** - this will exit the program

## Server
After starting the server you need to provide ip and port, default is `localhost:111` and then server is ready to listen all connections

