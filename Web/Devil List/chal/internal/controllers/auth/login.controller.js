const express = require('express');

const login = require('../../services/auth/login.service');
const { generate } = require('../../services/token.service');

const router = express.Router();

router.post('/login', async (req, res) => {
    const { id, pw } = req.body;

    try {
        const result = await login(id, pw);

        if (result.success) {
            const token = await generate(result.id, result.role);

            return res.status(200).json({ success: true, token });
        } else {
            return res.status(400).json({ success: false });
        }
    } catch (err) {
        console.log(err)
        return res.status(500).send('server error');
    }

})

module.exports = router;