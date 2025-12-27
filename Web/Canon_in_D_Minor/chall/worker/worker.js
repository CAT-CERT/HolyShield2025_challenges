const express = require('express');
const path = require('path');
const { HOSTNAME } = require('./src/config/env');
const userRoutes = require('./src/routes/userRoutes');
const adminRoutes = require('./src/routes/adminRoutes');
const { generateTicket, storeTicket } = require('./src/services/tickets');
const { setCooldown } = require('./src/services/cooldown');
const { COOKIE_NAME } = require('./src/config/constants');

const app = express();
const port = 8080;

app.use(express.json());
app.use(express.static(path.join(__dirname, 'public', 'html')));
app.use('/css', express.static(path.join(__dirname, 'public', 'css')));
app.use('/js', express.static(path.join(__dirname, 'public', 'js')));

app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'html', 'index.html'));
});

app.get('/register', async (_req, res) => {
    const ticket = generateTicket();
    await storeTicket(ticket);
    setCooldown(ticket);
    res.setHeader('Set-Cookie', `${COOKIE_NAME}=${ticket}; Path=/; HttpOnly`);
    res.status(200).json({ ticket });
});

app.use('/user', userRoutes);
app.use('/admin', adminRoutes);

app.listen(port, () => {
    console.log(`Worker (${HOSTNAME}) listening on port ${port}`);
});
