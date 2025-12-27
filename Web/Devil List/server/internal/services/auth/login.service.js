const pool = require('../db.service');

const login = async (id, pw) => {
    const conn = await pool.getConnection(async conn => conn);

    try {
        const [rows] = await conn.execute(
            'SELECT * FROM users WHERE id = ?',
            [id]
        );

        if (rows.length === 0) {
            return { success: false, message: 'User not found' };
        }

        const user = rows[0];

        if (pw !== user.pw) {
            return { success: false, message: 'Invalid password' };
        }

        return { success: true, id: user.id, role: user.role };
    } catch (err) {
        return { success: false, message: 'Internal error' };
    } finally {
        conn.release();
    }
};

module.exports = login;
