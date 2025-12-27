const express = require("express");
const cookieParser = require('cookie-parser');
const cors = require('cors');

const authRouter = require('./routers/auth.router');
const devilRouter = require('./routers/devil.router');

const HOST = "0.0.0.0";
const PORT = 8000;
const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());

app.use(cors({
    origin: 'http://localhost:3000',
    credentials: true
}));

app.use(authRouter);
app.use(devilRouter);

app.listen(PORT, HOST, () => {
    console.log(`Server is running at http://${HOST}:${PORT}`);
})