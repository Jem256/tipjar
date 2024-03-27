'use client';
import React, { useState } from 'react';
import { signUp } from '../services/api';
import { useDispatch } from 'react-redux';
import { register } from '../services/actions';
import { useRouter } from 'next/navigation';

const SignUp: React.FC = () => {
    const dispatch = useDispatch();
    const router = useRouter();

    const [formData, setFormData] = useState({
        name: '',
        email: '',
        password: '',
        // confirmPassword: '',
    });

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFormData({
            ...formData,
            [e.target.name]: e.target.value,
        });
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        try {
            const response = await signUp(formData);
            if (response) {
                dispatch(register());
                router.push('/login');
            }
        } catch (error) {
            console.error(error); 
        }
    };
    return (
        <div className='px-4 sm:px-6 lg:px-8'>
            <div className='max-w-md w-full space-y-8'>
                <div>
                    <h2 className='mt-6 text-center text-3xl font-extrabold text-gray-900'>
                        Sign up for an account
                    </h2>
                </div>
                <form className='mt-8 space-y-6' onSubmit={handleSubmit}>
                    <input type='hidden' name='remember' value='true' />
                    <div className='rounded-md shadow-sm space-y-4'>
                        <div>
                            <label htmlFor='name' className='sr-only'>
                                Full Name
                            </label>
                            <input
                                id='name'
                                name='name'
                                type='text'
                                autoComplete='name'
                                required
                                className='appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Full Name'
                                onChange={handleInputChange}
                            />
                        </div>
                        <div>
                            <label htmlFor='emailAddress' className='sr-only'>
                                Email Address
                            </label>
                            <input
                                id='emailAddress'
                                name='email'
                                type='email'
                                autoComplete='email'
                                required
                                className='appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Email address'
                                onChange={handleInputChange}
                            />
                        </div>
                        <div>
                            <label htmlFor='password' className='sr-only'>
                                Password
                            </label>
                            <input
                                id='password'
                                name='password'
                                type='password'
                                autoComplete='new-password'
                                required
                                className='appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Password'
                                onChange={handleInputChange}
                            />
                        </div>
                        {/* <div>
                            <label
                                htmlFor='confirmPassword'
                                className='sr-only'
                            >
                                Confirm Password
                            </label>
                            <input
                                id='confirmPassword'
                                name='confirmPassword'
                                type='password'
                                autoComplete='new-password'
                                required
                                className='appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Confirm Password'
                                onChange={handleInputChange}
                            />
                        </div> */}
                    </div>

                    <div>
                        <button
                            type='submit'
                            className='group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-gray-600 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500'
                        >
                            Sign up
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};

export default SignUp;
