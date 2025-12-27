import { getCookie } from 'h3'
import axios from 'axios'

export default defineEventHandler(async (event) => {
    const config = useRuntimeConfig()
    const name = event.context.params!.name
    const token = getCookie(event, 'token')

    const url = `http://${config.internalHost}:${config.internalPort}/devil/view`

    try {
        const res = await axios.post(url,
            {
                name
            },
            {
                headers: {
                    'Content-Type': 'application/json',
                    Cookie: `token=${token}`
                },
                withCredentials: true
            }
        )

        if (res.data.success) {
            return res.data.devil
        } else {
            throw createError({ statusCode: 500, statusMessage: 'Server Error' })
        }
    } catch (err) {
        console.error(err)
        throw createError({ statusCode: 500, statusMessage: 'Server Error' })
    }
})
