const _ = require('lodash');

const devils = {
    'Satan': 'Satan is the leader of the fallen angels, described in Scripture as the \'tempter,\' the \'father of lies,\' and the \'accuser.\' He seeks to turn humanity away from God, and in the Book of Revelation he is depicted as being cast out of heaven by Saint Michael the Archangel and his angels. (CCC 391-395, 414; Revelation 12:7-9)',
    'Beelzebul': 'Beelzebul is referred to in the New Testament as the \'prince of demons\' (Matthew 12:24) and is understood either as identical with Satan or as a representative of his power. He is associated with causing confusion and division, but is shown to be powerless before the authority of Jesus and the power of God. (Matthew 12:24-29; CCC 550).',
    'Abaddon': 'Abaddon, meaning \'destruction\' in Hebrew, and Apollyon, meaning \'destroyer\' in Greek, is named in Revelation 9:11 as the angel of the bottomless pit. He symbolizes the destructive and death-dealing powers that threaten humanity, yet can only act within the limits of God\'s providence. (Revelation 9:11)'
};

const devilWriter = {
    'Satan': 'admin',
    'Beelzebul': 'admin',
    'Abaddon': 'admin'
};

const devilDetailRoutes = {
    'Satan': '/devil/Satan',
    'Beelzebul': '/devil/Beelzebul',
    'Abaddon': '/devil/Abaddon'
};

const setDevil = (devilName, description, writer) => {
    _.set(devils, devilName, description);

    if (!devilDetailRoutes[devilName]) {
        setDevilDetailRoutes(devilName);
    }

    devilWriter[devilName] = writer;

    console.log(devils);
    console.log(devilDetailRoutes);
}

const getDevils = (writer) => {
    if (writer === 'admin') {
        return devils;
    }

    const filteredDevils = {};
    for (const name in devils) {
        if (devilWriter[name] === writer) {
            filteredDevils[name] = devils[name];
        }
    }

    return filteredDevils;
}

const getDevil = (name, writer) => {
    if (writer === 'admin') {
        return devils[name];
    }

    if (devilWriter[name] !== writer) {
        return;
    }

    return devils[name];
}

const setDevilDetailRoutes = (name) => {
    devilDetailRoutes[name] = `/devil/${name}`;
}

const getDevilDetailRoutes = (writer) => {
    const resDetailRoutes = {};

    if (writer === 'admin') {
        for (const k in devilDetailRoutes) {
            if (typeof devilDetailRoutes[k] === 'string') {
                resDetailRoutes[k] = devilDetailRoutes[k];
            }
        }
    }
    else {
        for (const k in devilDetailRoutes) {
            if (devilWriter[k] === writer && typeof devilDetailRoutes[k] === 'string') {
                resDetailRoutes[k] = devilDetailRoutes[k];
            }
        }
    }

    return resDetailRoutes;
}

module.exports = { devils, devilWriter, devilDetailRoutes, setDevil, getDevils, getDevil, getDevilDetailRoutes };
