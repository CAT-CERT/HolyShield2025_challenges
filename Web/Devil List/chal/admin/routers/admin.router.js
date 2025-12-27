const express = require('express');

const checkRouter = require('../controllers/check.controller');

const router = express.Router();

router.use('/admin', checkRouter);

module.exports = router;