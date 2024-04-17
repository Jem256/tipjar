import { UserData } from '../types';
export const login = (userData: UserData) => ({
    type: 'LOGIN',
    payload: userData,
});

export const register = () => ({
    type: 'REGISTER',
});

export const logout = () => ({
    type: 'LOGOUT',
});
