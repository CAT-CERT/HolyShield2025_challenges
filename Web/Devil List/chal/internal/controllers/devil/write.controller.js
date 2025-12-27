const express = require('express');
const { write } = require('../../services/devil/write.service');
const { checkWriter } = require('../../middlewares/checkAuth.middleware');

const router = express.Router();

router.post('/write', checkWriter, async (req, res) => {
    const { devilName, description } = req.body;

    try {
        const result = await write(devilName, description, req.user.id);

        if (result.success) {
            return res.status(200).json({ success: true });
        } else if (result.message === 'Unauthorized') {
            return res.status(401).json({ success: false });
        } else {
            return res.status(500).json({ success: false });
        }
    } catch (err) {
        console.error(err);
        return res.status(500).json({ success: false });
    }
});

module.exports = router;
