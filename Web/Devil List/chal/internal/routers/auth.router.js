const express = require('express');

const registerController = require('../controllers/auth/register.controller');
const loginController = require('../controllers/auth/login.controller');

const router = express.Router();

router.use('/auth', registerController);
router.use('/auth', loginController);

module.exports = router;