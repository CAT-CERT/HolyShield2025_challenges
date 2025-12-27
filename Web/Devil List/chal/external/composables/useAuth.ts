import { jwtDecode } from 'jwt-decode'

export const useAuth = () => {
    const token = useCookie<string | null>('token').value

    if (!token) {
        return { login: false }
    }

    try {
        const payload = jwtDecode<{ id?: string }>(token)

        if (!payload || !payload.id) {
            return { login: false }
        }

        return { login: true }
    } catch {
        return { login: false }
    }
}
