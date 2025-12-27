const express = require('express');
const app = express();

app.disable('x-powered-by');

app.use((req, res, next) => {
  const ban = req.get('X-Access-Denied');
  if (ban) {
    res.set('Connection', 'close');
    res.status(403).type('text/plain').send('Banned\n');
    res.shouldKeepAlive = false;
    req.socket.setKeepAlive?.(false);
    res.on('finish', () => { try { req.socket.destroy(); } catch {} });
    return;
  }
  next();
});


app.get('/login', (req, res) => {
  const id  = req.query.id  || '';
  const pw  = req.query.pw  || '';

  const users = {
    admin: 'wermutw3rmutwermu7w3rmu7',
    guest: 'guest'
  };
  const tokens = {
    admin: 'wermutw3rmutwermu7w3rmu7wermutw3rmutwermu7w3rmu7',
    guest: 'guest'
  };

  if (!users[id] || users[id] !== pw) {
    return res.status(401).type('text/plain').send('Invalid credentials\n');
  }

  res.cookie('auth', tokens[id], { httpOnly: true });
  res.type('text/plain').send(`Welcome ${id}\n`); 
});


app.get('/admin', (req, res) => {
  res.type('text/plain').send('hello admin!\n');
});


app.get('/admin/status', (req, res) => {
  const cache = String(req.query?.cache ?? 'no-cache');
  const sock = req.socket || res.socket;
  try {
    sock.write('HTTP/1.1 200 OK\r\n');
    sock.write('Content-Type: application/json; charset=utf-8\r\n');
    sock.write('Cache-Control: ' + cache + '\r\n');   
    sock.write('Connection: close\r\n');
    sock.write('\r\n');
    sock.write(JSON.stringify({ status: "running", cache }), 'utf8');
    sock.end();
  } catch (e) {
    res.status(500).end();
  }
});


app.use((req, res) => {
  res.status(404).type('text/plain').send('Not Found\n');
});


app.listen(8088, () => console.log('Backend listening on 8088'));
