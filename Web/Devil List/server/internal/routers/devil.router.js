const express = require('express');

const devilListController = require('../controllers/devil/list.controller');
const devilWriteController = require('../controllers/devil/write.controller');
const devilViewController = require('../controllers/devil/view.controller');
const devilArchdioceseController = require('../controllers/devil/archdiocese.controller');

const router = express.Router();

router.use('/devil', devilListController);
router.use('/devil', devilWriteController);
router.use('/devil', devilViewController);
router.use('/devil', devilArchdioceseController);

module.exports = router;