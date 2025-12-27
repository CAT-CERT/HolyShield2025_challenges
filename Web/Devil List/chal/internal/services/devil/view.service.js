const { getDevil, getDevils } = require('../../models/devil.model');

const view = async (name, id) => {
    try {
        const devilList = getDevils(id);

        if (!devilList) {
            return { success: false, status: 403 };
        }

        const devil = await getDevil(name, id);

        if (!devil) {
            return { success: false, status: 404 };
        }

        return { success: true, devil };
    } catch (err) {
        console.error(err)
        return { success: false, message: 'Internal error' };
    }
};

module.exports = view;
