import { getCookie, readBody } from 'h3'
import axios from 'axios'

export default defineEventHandler(async (event) => {
    let { devilName, description } = await readBody(event)

    if (!devilName || !description) {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    if (description.includes('\\')) {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    const config = useRuntimeConfig()

    const url = `http://${config.internalHost}:${config.internalPort}/devil/write`

    try {
        const token = getCookie(event, 'token')

        const res = await axios.post(url,
            {
                devilName,
                description
            },
            {
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                },
                withCredentials: true
            }
        )

        if (res.status === 401) {
            throw createError({ statusCode: 401, statusMessage: 'Unauthorized' })
        } else if (res.status === 200) {
            return { message: 'success' }
        }
    } catch (err) {
        throw createError({ statusCode: 500, statusMessage: 'Server Error' })
    }
})
