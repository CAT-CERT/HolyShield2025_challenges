const { getDevils, getDevilDetailRoutes } = require('../../models/devil.model');

const list = (writer) => {
    const devilList = getDevils(writer);
    const devilRoutes = getDevilDetailRoutes(writer);

    return { devilList, devilRoutes };
};

module.exports = list;
