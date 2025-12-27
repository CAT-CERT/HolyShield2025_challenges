const express = require('express');
const view = require('../../services/devil/view.service');
const { checkWriter } = require('../../middlewares/checkAuth.middleware');

const router = express.Router();

router.post('/view', checkWriter, async (req, res) => {
    const { name } = req.body;

    try {
        const result = await view(name, req.user.id);

        if (result.success) {
            return res.status(200).json({ success: true, devil: result.devil });
        } else if (result.status === 403) {
            return res.status(403).json({ success: false });
        } else if (result.status === 404) {
            return res.status(404).json({ success: false });
        } else {
            return res.status(500).json({ success: false });
        }
    } catch (e) {
        console.error(e);
        return res.status(401).json({ success: false });
    }
});

module.exports = router;
