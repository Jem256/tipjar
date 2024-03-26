import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:8000';

export const signUp = async (userData: any) => {
    try {
        const response = await axios.post(`${API_BASE_URL}/register`, userData);
        return response;
    } catch (error) {
        throw error;
    }
};

export const logIn = async (userData: any) => {
    try {
        const response = await axios.post(`${API_BASE_URL}/login`, userData);
        return response.data;
    } catch (error) {
        throw error;
    }
};
