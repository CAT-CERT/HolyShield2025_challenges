import { getCookie } from 'h3'
import axios from 'axios'

export default defineEventHandler(async (event) => {
    const config = useRuntimeConfig()
    const token = getCookie(event, 'token')

    const url = `http://${config.internalHost}:${config.internalPort}/devil/`

    try {
        const res = await axios.get(url,
            {
                headers: {
                    Cookie: `token=${token}`
                },
                withCredentials: true
            }
        )

        return { devils: res.data.devilList, devilRoutes: res.data.devilRoutes }
    } catch (err) {
        console.error(err)
        throw createError({ statusCode: 500, statusMessage: 'Server Error' })
    }
})
