import { UserData } from '../types';

// reducers/authReducer.js
export interface AuthState {
    user: UserData | null;
}

const initialState: AuthState = {
    user: null,
};

const authReducer = (
    state = initialState,
    action: { type: any; payload: any }
) => {
    switch (action.type) {
        case 'LOGIN':
            return {
                ...state,
                user: action.payload,
            };
        case 'REGISTER':
            return {
                ...state,
                user: action.payload,
            };
        case 'LOGOUT':
            return {
                ...state,
                user: null,
            };
        default:
            return state;
    }
};

export default authReducer;
