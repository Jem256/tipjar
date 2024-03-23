// pages/signup.tsx

import React from 'react';

const SignUp: React.FC = () => {
    return (
        <div className='px-4 sm:px-6 lg:px-8'>
            <div className='max-w-md w-full space-y-8'>
                <div>
                    <h2 className='mt-6 text-center text-3xl font-extrabold text-gray-900'>
                        Sign up for an account
                    </h2>
                </div>
                <form className='mt-8 space-y-6' action='#' method='POST'>
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
                            />
                        </div>
                        <div>
                            <label htmlFor='email-address' className='sr-only'>
                                Email address
                            </label>
                            <input
                                id='email-address'
                                name='email'
                                type='email'
                                autoComplete='email'
                                required
                                className='appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Email address'
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
                            />
                        </div>
                        <div>
                            <label
                                htmlFor='confirm-password'
                                className='sr-only'
                            >
                                Confirm Password
                            </label>
                            <input
                                id='confirm-password'
                                name='confirm-password'
                                type='password'
                                autoComplete='new-password'
                                required
                                className='appearance-none rounded relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm'
                                placeholder='Confirm Password'
                            />
                        </div>
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
