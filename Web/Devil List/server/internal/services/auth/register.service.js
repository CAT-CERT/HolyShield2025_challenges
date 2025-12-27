const pool = require('../db.service');

const register = async (id, pw, role) => {
    const conn = await pool.getConnection(async conn => conn);

    try {
        const [rows] = await conn.execute(
            'SELECT id FROM users WHERE id = ?',
            [id]
        );

        if (rows.length > 0) {
            return { success: false, message: 'ID already exists' };
        }

        await conn.execute(
            'INSERT INTO users (id, pw, role) VALUES (?, ?, ?)',
            [id, pw, role]
        );

        return { success: true };
    } catch (err) {
        return { success: false, message: 'Internal error' };
    } finally {
        conn.release();
    }
}

module.exports = register;