const express = require('express');
const { checkInquisitor } = require('../../middlewares/checkAuth.middleware');

const router = express.Router();

router.post('/archdiocese', checkInquisitor, async (req, res) => {
    const { devilName } = req.body;

    return res.status(200).json({ success: true, devilName: devilName });
});

module.exports = router;