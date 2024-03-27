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
    const headers = {
        'Content-Type': 'application/json',
    };
    try {
        const response = await axios.post(`${API_BASE_URL}/login`, userData, {
            headers,
        });
        return response.data;
    } catch (error) {
        throw error;
    }
};

export const payment = async (userData: any) => {
    const headers = {
        'Content-Type': 'application/json',
    };
    try {
        const response = await axios.post(
            `${API_BASE_URL}/generate_invoice/6c696e646140746573742e636f6d`,
            userData,
            {
                headers,
            }
        );
        return response.data;
    } catch (error) {
        throw error;
    }
};
