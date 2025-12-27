const translations = {
    en: {
        title: "Developing...",
        username: "Username:",
        password: "Password:",
        login: "Login",
        languageSettings: "Language Settings",
        settingLanguage: "Setting language...",
        languageSetSuccess: "Language set to English successfully!",
        languageSetFailed: "Failed to set language:",
        connectionError: "Connection error:",
        loginSuccess: "Login successful! Welcome",
        loginFailed: "Login failed:",
        invalidCredentials: "Invalid username or password"
    },
    ko: {
        title: "\uac1c\ubc1c \uc911...",
        username: "\uc0ac\uc6a9\uc790\uba85:",
        password: "\ube44\ubc00\ubc88\ud638:",
        login: "\ub85c\uadf8\uc778",
        languageSettings: "\uc5b8\uc5b4 \uc124\uc815",
        settingLanguage: "\uc5b8\uc5b4 \uc124\uc815 \uc911...",
        languageSetSuccess: "\uc5b8\uc5b4\uac00 \ud55c\uad6d\uc5b4\ub85c \uc124\uc815\ub418\uc5c8\uc2b5\ub2c8\ub2e4!",
        languageSetFailed: "\uc5b8\uc5b4 \uc124\uc815 \uc2e4\ud328:",
        connectionError: "\uc5f0\uacb0 \uc624\ub958:",
        loginSuccess: "\ub85c\uadf8\uc778 \uc131\uacf5! \ud658\uc601\ud569\ub2c8\ub2e4",
        loginFailed: "\ub85c\uadf8\uc778 \uc2e4\ud328:",
        invalidCredentials: "\uc798\ubabb\ub41c \uc0ac\uc6a9\uc790\uba85 \ub610\ub294 \ube44\ubc00\ubc88\ud638"
    },
    ja: {
        title: "\u958b\u767a\u4e2d...",
        username: "\u30e6\u30fc\u30b6\u30fc\u540d:",
        password: "\u30d1\u30b9\u30ef\u30fc\u30c9:",
        login: "\u30ed\u30b0\u30a4\u30f3",
        languageSettings: "\u8a00\u8a9e\u8a2d\u5b9a",
        settingLanguage: "\u8a00\u8a9e\u3092\u8a2d\u5b9a\u3057\u3066\u3044\u307e\u3059...",
        languageSetSuccess: "\u8a00\u8a9e\u304c\u65e5\u672c\u8a9e\u306b\u8a2d\u5b9a\u3055\u308c\u307e\u3057\u305f\uff01",
        languageSetFailed: "\u8a00\u8a9e\u8a2d\u5b9a\u306b\u5931\u6557\u3057\u307e\u3057\u305f:",
        connectionError: "\u63a5\u7d9a\u30a8\u30e9\u30fc:",
        loginSuccess: "\u30ed\u30b0\u30a4\u30f3\u6210\u529f! \u3088\u3046\u3053\u305d",
        loginFailed: "\u30ed\u30b0\u30a4\u30f3\u5931\u6557:",
        invalidCredentials: "\u7121\u52b9\u306a\u30e6\u30fc\u30b6\u30fc\u540d\u307e\u305f\u306f\u30d1\u30b9\u30ef\u30fc\u30c9"
    },
    zh: {
        title: "\u5f00\u53d1\u4e2d...",
        username: "\u7528\u6237\u540d:",
        password: "\u5bc6\u7801:",
        login: "\u767b\u5f55",
        languageSettings: "\u8bed\u8a00\u8bbe\u7f6e",
        settingLanguage: "\u6b63\u5728\u8bbe\u7f6e\u8bed\u8a00...",
        languageSetSuccess: "\u8bed\u8a00\u5df2\u6210\u529f\u8bbe\u7f6e\u4e3a\u4e2d\u6587\uff01",
        languageSetFailed: "\u8bed\u8a00\u8bbe\u7f6e\u5931\u8d25:",
        connectionError: "\u8fde\u63a5\u9519\u8bef:",
        loginSuccess: "\u767b\u5f55\u6210\u529f! \u6b22\u8fce",
        loginFailed: "\u767b\u5f55\u5931\u8d25:",
        invalidCredentials: "\u65e0\u6548\u7684\u7528\u6237\u540d\u6216\u5bc6\u7801"
    }
};

let currentLanguage = 'en';
function changeLanguage(lang) {
    const texts = translations[lang];

    if (!texts) {
        console.error('Language not supported:', lang);
        return;
    }

    currentLanguage = lang;

    document.querySelectorAll('[data-text]').forEach(element => {
        const key = element.getAttribute('data-text');
        if (texts[key]) {
            if (element.tagName === 'INPUT' && element.type === 'submit') {
                element.value = texts[key];
            } else {
                element.textContent = texts[key];
            }
        }
    });

    document.querySelectorAll('.lang-btn').forEach(btn => {
        btn.classList.remove('active');
        if (btn.getAttribute('data-lang') === lang) {
            btn.classList.add('active');
        }
    });

    if (typeof(Storage) !== "undefined") {
        try {
            localStorage.setItem('preferredLanguage', lang);
        } catch(e) {
            console.warn('Failed to save preferred language:', e);
        }
    }

    console.log('Language changed to:', lang);
}

function setLanguage(lang) {
    const resultDiv = document.getElementById('language-result');
    const texts = translations[currentLanguage];

    if (!resultDiv) {
        console.error('Result div not found');
        return;
    }

    resultDiv.style.display = 'block';
    resultDiv.innerHTML = texts.settingLanguage;
    resultDiv.className = 'result loading';

    fetch('/', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            method: "setLanguage",
            params: {language: lang}
        })
    })
    .then(response => {
        if (!response.ok) throw new Error('HTTP ' + response.status);
        return response.json();
    })
    .then(data => {
        if (data.error_code === 0) {
            changeLanguage(lang);
            const newTexts = translations[lang];
            resultDiv.innerHTML = newTexts.languageSetSuccess;
            resultDiv.className = 'result success';
        } else {
            const currentTexts = translations[currentLanguage];
            resultDiv.innerHTML = currentTexts.languageSetFailed + ' ' + (data.message || 'Unknown error');
            resultDiv.className = 'result error';
        }
    })
    .catch(error => {
        console.error('Language setting error:', error);
        const currentTexts = translations[currentLanguage];
        resultDiv.innerHTML = currentTexts.connectionError + ' ' + error.message;
        resultDiv.className = 'result error';
    });

    setTimeout(() => {
        if (resultDiv) resultDiv.style.display = 'none';
    }, 5000);
}

const LANGUAGE_ALIASES = {
    en: 'en',
    english: 'en',
    eng: 'en',
    ko: 'ko',
    kor: 'ko',
    korean: 'ko',
    ja: 'ja',
    jp: 'ja',
    jpn: 'ja',
    japanese: 'ja',
    zh: 'zh',
    cn: 'zh',
    chi: 'zh',
    chn: 'zh',
    chinese: 'zh'
};

const LANGUAGE_LABELS = {
    en: 'English (en)',
    ko: 'Korean (ko)',
    ja: 'Japanese (ja)',
    zh: 'Chinese (zh)'
};

const PROMPT_PREFIX = 'user@avotapo:~$';
const LOGIN_EXPECTED = { username: 'avotapo', password: 'happy' };

let loginState = 'idle';
let pendingLogin = { username: '', password: '' };
let loggedInUser = null;

const commandHistory = [];
let historyIndex = 0;

function getTerminalOutput() {
    return document.getElementById('terminal-output');
}

function clearTerminalOutput() {
    const output = getTerminalOutput();
    if (!output) return;
    while (output.firstChild) {
        output.removeChild(output.firstChild);
    }
}

function clean() {
    clearTerminalOutput();
}

function test() {
    clearTerminalOutput();
    appendOutput('Developing...');
}

function appendOutput(text, type = 'default') {
    const output = getTerminalOutput();
    if (!output) return;

    const line = document.createElement('div');
    line.className = 'output-line' + (type && type !== 'default' ? ` ${type}` : '');
    line.textContent = text;
    output.appendChild(line);
    output.scrollTop = output.scrollHeight;
}

function appendCommandEcho(commandText) {
    const output = getTerminalOutput();
    if (!output) return;

    const line = document.createElement('div');
    line.className = 'output-line prompt';

    const prefix = document.createElement('span');
    prefix.className = 'prompt-prefix';
    prefix.textContent = PROMPT_PREFIX;

    const command = document.createElement('span');
    command.className = 'command-text';
    command.textContent = commandText;

    line.appendChild(prefix);
    line.appendChild(command);
    output.appendChild(line);
    output.scrollTop = output.scrollHeight;
}

function appendPasswordEcho(length) {
    const output = getTerminalOutput();
    if (!output) return;

    const line = document.createElement('div');
    line.className = 'output-line prompt';

    const prefix = document.createElement('span');
    prefix.className = 'prompt-prefix';
    prefix.textContent = PROMPT_PREFIX;

    const masked = document.createElement('span');
    masked.className = 'command-text';
    masked.textContent = '*'.repeat(Math.max(length, 1));

    line.appendChild(prefix);
    line.appendChild(masked);
    output.appendChild(line);
    output.scrollTop = output.scrollHeight;
}

function appendHelp() {
    appendOutput('Available commands:', 'info');
    appendOutput('  help                Show this help message', 'info');
    appendOutput('  login               Authenticate as avotapo', 'info');
    appendOutput('  clear               Clear terminal output', 'info');
    appendOutput('  test                Show Developing... message', 'info');
    appendOutput('  setlanguage <code>  Change UI language (see --help)', 'info');
}

function handleSetLanguage(args) {
    if (!args.length) {
        appendOutput('Usage: setlanguage <code> | setlanguage --help', 'info');
        return;
    }

    const firstArg = args[0].toLowerCase();
    if (firstArg === '--help') {
        appendOutput('Supported languages:', 'info');
        Object.entries(LANGUAGE_LABELS).forEach(([code, label]) => {
            appendOutput(`  ${label}`, 'info');
        });
        appendOutput('Use: setlanguage <code>', 'info');
        return;
    }

    const normalized = LANGUAGE_ALIASES[firstArg] || firstArg;
    if (!translations[normalized]) {
        appendOutput(`Language "${firstArg}" not supported. Type setlanguage --help for a list.`, 'error');
        return;
    }

    setLanguage(normalized);
}

function handleLoginCommand(args) {
    if (loggedInUser) {
        appendOutput(`Already logged in as ${loggedInUser}.`, 'info');
        return;
    }

    if (args.length === 1 && args[0].toLowerCase() === '--help') {
        appendOutput('Usage: login', 'info');
        appendOutput('Follow prompts for username and password.', 'info');
        appendOutput('Or provide credentials inline: login <username> <password>', 'info');
        return;
    }

    if (args.length >= 2) {
        validateLogin(args[0], args[1]);
        return;
    }

    const texts = translations[currentLanguage] || translations.en;
    appendOutput(texts.username || 'Username:', 'info');
    loginState = 'awaitingUsername';
}

function validateLogin(usernameInput, passwordInput) {
    const texts = translations[currentLanguage] || translations.en;
    const username = (usernameInput || '').trim();
    const password = (passwordInput || '').trim();

    if (username === LOGIN_EXPECTED.username && password === LOGIN_EXPECTED.password) {
        loggedInUser = username;
        appendOutput(`${texts.loginSuccess || 'Login successful!'} ${username}!`, 'success');
    } else {
        appendOutput(`${texts.loginFailed || 'Login failed:'} ${texts.invalidCredentials || 'Invalid username or password'}`, 'error');
    }
}

function registerHistory(command) {
    if (!command) return;
    const last = commandHistory[commandHistory.length - 1];
    if (last === command) {
        historyIndex = commandHistory.length;
        return;
    }
    commandHistory.push(command);
    historyIndex = commandHistory.length;
}

function processLoginFlow(inputValue) {
    const texts = translations[currentLanguage] || translations.en;

    if (loginState === 'awaitingUsername') {
        const username = inputValue.trim();
        if (!username) {
            appendOutput(texts.invalidCredentials || 'Invalid username or password', 'error');
            loginState = 'idle';
            pendingLogin = { username: '', password: '' };
            return;
        }
        pendingLogin.username = username;
        appendOutput(texts.password || 'Password:', 'info');
        loginState = 'awaitingPassword';
        return;
    }

    if (loginState === 'awaitingPassword') {
        loginState = 'idle';
        validateLogin(pendingLogin.username, inputValue);
        pendingLogin = { username: '', password: '' };
    }
}

function processCommand(rawInput) {
    const trimmedInput = rawInput.trim();

    if (loginState === 'awaitingUsername') {
        appendCommandEcho(rawInput);
        processLoginFlow(trimmedInput);
        return;
    }

    if (loginState === 'awaitingPassword') {
        appendPasswordEcho(rawInput.length);
        processLoginFlow(rawInput);
        return;
    }

    if (!trimmedInput) {
        return;
    }

    appendCommandEcho(trimmedInput);
    registerHistory(trimmedInput);

    const [command, ...args] = trimmedInput.split(/\s+/);
    const lowerCommand = command.toLowerCase();

    switch (lowerCommand) {
        case 'help':
            appendHelp();
            break;
        case 'setlanguage':
            handleSetLanguage(args);
            break;
        case 'login':
            handleLoginCommand(args);
            break;
        case 'clear':
            clean();
            break;
        case 'test':
            test();
            break;
        default:
            appendOutput(`Command "${command}" not found. Type help to list commands.`, 'error');
            break;
    }
}

function setupCommandInterface() {
    const commandForm = document.getElementById('commandForm');
    const commandInput = document.getElementById('command-input');
    if (!commandForm || !commandInput) return;

    historyIndex = commandHistory.length;
    commandInput.focus();

    commandForm.addEventListener('submit', event => {
        event.preventDefault();
        const value = commandInput.value;
        processCommand(value);
        commandInput.value = '';
        historyIndex = commandHistory.length;
        commandInput.focus();
    });

    commandInput.addEventListener('keydown', event => {
        if (event.key === 'ArrowUp') {
            if (!commandHistory.length) return;
            event.preventDefault();
            if (historyIndex > 0) {
                historyIndex -= 1;
            } else {
                historyIndex = commandHistory.length - 1;
            }
            commandInput.value = commandHistory[historyIndex] || '';
            setTimeout(() => {
                const end = commandInput.value.length;
                commandInput.setSelectionRange(end, end);
            }, 0);
        } else if (event.key === 'ArrowDown') {
            if (!commandHistory.length) return;
            event.preventDefault();
            if (historyIndex < commandHistory.length - 1) {
                historyIndex += 1;
                commandInput.value = commandHistory[historyIndex];
            } else {
                historyIndex = commandHistory.length;
                commandInput.value = '';
            }
            setTimeout(() => {
                const end = commandInput.value.length;
                commandInput.setSelectionRange(end, end);
            }, 0);
        }
    });
}

function observeLanguageResult() {
    const resultDiv = document.getElementById('language-result');
    if (!resultDiv) return;

    let lastMessage = '';
    const observer = new MutationObserver(() => {
        const text = resultDiv.textContent.trim();
        if (!text) {
            lastMessage = '';
            return;
        }

        if (text === lastMessage) {
            return;
        }
        lastMessage = text;

        let type = 'default';
        if (resultDiv.classList.contains('error')) {
            type = 'error';
        } else if (resultDiv.classList.contains('success')) {
            type = 'success';
        } else if (resultDiv.classList.contains('loading')) {
            type = 'info';
        }

        appendOutput(text, type);
    });

    observer.observe(resultDiv, { childList: true, attributes: true, characterData: true, subtree: true });
}

document.addEventListener('DOMContentLoaded', () => {
    console.log('DOM loaded, initializing...');

    let initialLanguage = 'en';

    if (typeof(Storage) !== "undefined") {
        const savedLanguage = localStorage.getItem('preferredLanguage');
        if (savedLanguage && translations[savedLanguage]) {
            initialLanguage = savedLanguage;
        } else {
            const navLang = navigator.language || navigator.userLanguage || '';
            const browserLang = navLang.split('-')[0];
            if (translations[browserLang]) {
                initialLanguage = browserLang;
            }
        }
    }

    changeLanguage(initialLanguage);

    setupCommandInterface();
    observeLanguageResult();

    console.log('Initialization complete, current language:', initialLanguage);
});


