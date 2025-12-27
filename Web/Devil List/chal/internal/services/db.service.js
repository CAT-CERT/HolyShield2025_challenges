const mysql = require("mysql2/promise");
require("dotenv").config({ path: __dirname + "/../config/.env" });

const mysqlHost = process.env.MYSQL_HOST;
const mysqlUsername = process.env.MYSQL_USERNAME;
const mysqlPassword = process.env.MYSQL_PASSWORD;
const mysqlDB = process.env.MYSQL_DATABASE;

const pool = mysql.createPool({
    host : mysqlHost,
    user : mysqlUsername,
    password : mysqlPassword,
    database : mysqlDB,
    connectionLimit : 10,
    waitForConnections: true,
    queueLimit: 0,
});

module.exports = pool;