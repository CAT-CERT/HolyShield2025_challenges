const express = require('express');

const router = require('./routers/admin.router');

const HOST = "0.0.0.0";
const PORT = 6000;
const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: false }));

app.use(router);

app.listen(PORT, HOST, () => {
    console.log(`Server is running at http://${HOST}:${PORT}`);
})