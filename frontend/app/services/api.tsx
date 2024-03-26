import axios from 'axios';

const API_BASE_URL = 'https://example.com/api'; // Replace this with your API base URL

export const signUp = async (userData: any) => {
    try {
        const response = await axios.post(`${API_BASE_URL}/signup`, userData);
        return response.data;
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
