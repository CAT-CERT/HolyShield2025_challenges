const createDOMPurify = require('dompurify');
const { JSDOM } = require('jsdom');
const { setDevil } = require('../../models/devil.model');

const write = async (devilName, description, id) => {
    const { window } = new JSDOM('');
    const DOMPurify = createDOMPurify(window);

    try {
        const sanitizedDevilName = DOMPurify.sanitize(devilName);
        const sanitizedDescription = DOMPurify.sanitize(description);

        await setDevil(sanitizedDevilName, sanitizedDescription, id);

        return { success: true, sanitizedDevilName };
    } catch (err) {
        console.error(err);
        return { success: false, message: 'Internal error' };
    }
};

module.exports = { write };
