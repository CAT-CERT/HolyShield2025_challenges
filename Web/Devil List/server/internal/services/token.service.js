const jwt = require('jsonwebtoken');
require("dotenv").config({ path: __dirname + "/../config/.env" });

const generate = async (id, role) => {
    const token = await jwt.sign(
        { id: id, role: role },
        process.env.JWT_SECRET,
        { expiresIn: '1h' }
    );

    return token;
}

const verify = async (token) => {
    const data = await jwt.verify(
        token,
        process.env.JWT_SECRET
    );

    return data;
}

module.exports = {
    generate,
    verify
}
