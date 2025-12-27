const { verify } = require('../services/token.service');

const checkWriter = async (req, res, next) => {
    const token = req.cookies.token;

    try {
        const data = await verify(token);

        if (!data) {
            return res.status(401).json({ success: false });
        }

        if (typeof data.role !== 'string' || (data.role.toUpperCase() !== 'HUNTER' && data.role.toUpperCase() !== 'INQUISITOR')) {
            return res.status(403).json({ success: false });
        }

        req.user = data;
        next();
    } catch (err) {
        console.error(err);
        return res.status(500).json({ success: false });
    }
};

const checkInquisitor = async (req, res, next) => {
    const token = req.cookies.token;

    try {
        const data = await verify(token);

        if (!data) {
            return res.status(401).json({ success: false });
        }

        if (typeof data.role !== 'string' || data.role.toUpperCase() !== 'INQUISITOR') {
            return res.status(403).json({ success: false });
        }

        req.user = data;
        next();
    } catch (err) {
        console.error(err);
        return res.status(500).json({ success: false });
    }

};

module.exports = { checkWriter, checkInquisitor };