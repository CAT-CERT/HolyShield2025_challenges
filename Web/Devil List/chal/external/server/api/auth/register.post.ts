import { readBody } from 'h3'
import axios from 'axios'

export default defineEventHandler(async (event) => {
    const { id, pw, role } = await readBody(event)

    if (!id || !pw || !role) {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    if (typeof id !== 'string' || typeof pw !== 'string' || typeof role !== 'string') {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    if (!/^[A-Za-z0-9]+$/.test(id)) {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    if (role.toLowerCase() === 'inquisitor') {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    const config = useRuntimeConfig()

    const url = `http://${config.internalHost}:${config.internalPort}/auth/register`

    try {
        await axios.post(url, {
            id,
            pw,
            role
        },
        {
            headers: {
                'Content-Type': 'application/json'
            }
        })
        
        return { message: 'success' }
    } catch (err) {
        console.error(err)
        throw createError({ statusCode: 500, statusMessage: 'Server Error' });
    }

})
