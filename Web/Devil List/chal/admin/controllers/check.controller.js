const express = require('express');

const check = require('../services/check.service');

const router = express.Router();

router.post('/check', async (req, res) => {
    const { devilName } = req.body;

    try {
        const result = await check(devilName);

        if (result.success) {
            console.log('check');
        }

        return res.status(200).json({ success: true });
    } catch (e) {
        console.error(e);
        return res.status(500).json({ success: false });
    }
});

module.exports = router;