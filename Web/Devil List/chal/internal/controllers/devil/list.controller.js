const express = require('express');
const list = require('../../services/devil/list.service');
const { verify } = require('../../services/token.service');

const router = express.Router();

router.get('/', async (req, res) => {
    try {
        const token = req.cookies.token;
        const data = await verify(token);

        if (!data || !data.id) {
            return res.status(401).json({ success: false });
        }

        const result = await list(data.id);

        return res.status(200).json({ success: true, devilList: result.devilList, devilRoutes: result.devilRoutes });

    } catch (err) {
        return res.status(500).json({ success: false });
    }
});

module.exports = router;
