import { getCookie, readBody } from 'h3'
import axios from 'axios'

export default defineEventHandler(async (event) => {
    let { devilName } = await readBody(event)

    if (!devilName) {
        throw createError({ statusCode: 400, statusMessage: 'Bad Request' })
    }

    const config = useRuntimeConfig()

    const url = `http://${config.internalHost}:${config.internalPort}/devil/archdiocese`

    try {
        const token = getCookie(event, 'token')

        const res = await axios.post(url,
            {
                devilName
            },
            {
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                },
                withCredentials: true
            }
        )

        if (res.status === 200) {
                const adminUrl = `http://${config.adminHost}:${config.adminPort}/admin/check`

                const adminRes = await axios.post(adminUrl,
                    {
                        devilName: res.data.devilName
                    },
                    {
                        headers: {
                            'Content-Type': 'application/json'
                        }
                    }
                )

                if (adminRes.data.success) {
                    return { message: 'success' }
                } else {
                    throw createError({ statusCode: 500, statusMessage: 'Server Error' })
                }
        } else {
            throw createError({ statusCode: 500, statusMessage: 'Server Error' })
        }
    } catch (err) {
        throw createError({ statusCode: 500, statusMessage: 'Server Error' })
    }
})
