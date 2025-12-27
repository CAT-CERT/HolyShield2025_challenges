import { readBody, setCookie } from 'h3'
import axios from 'axios'

export default defineEventHandler(async (event) => {
    const { id, pw } = await readBody(event);

    if (!id || !pw) {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    if (typeof id !== 'string' || typeof pw !== 'string') {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    const config = useRuntimeConfig()

    const url = `http://${config.internalHost}:${config.internalPort}/auth/login`

    try {
        const res = await axios.post(url,
            {
                id,
                pw
            },
            {
                headers: {
                    'Content-Type': 'application/json'
                }
            }
        )
        if (res.data.success) {
            const token = res.data.token;

            setCookie(event, 'token', token)

            return { message: 'success' }
        } else {
            throw createError({ statusCode: 500, statusMessage: 'Server Error' })
        }
    } catch (err) {
        console.error(err);
        throw createError({ statusCode: 500, statusMessage: 'Server Error' })
    }
})