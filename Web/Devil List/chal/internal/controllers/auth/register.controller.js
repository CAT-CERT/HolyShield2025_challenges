const express = require('express');

const register = require('../../services/auth/register.service');

const router = express.Router();

router.post('/register', async (req, res) => {
    const { id, pw, role } = req.body;

    try {
        const result = await register(id, pw, role);

        if (result.success) {
            return res.status(200).json({ success: true });
        } else {
            return res.status(400).json({ success: false });
        }
    } catch (err) {
        console.log(err)
        return res.status(500).send('server error');
    }
})

module.exports = router;